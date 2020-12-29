use alloc::sync::{Arc, Weak};
use core::time::Duration;

use crate::{
    util::{
        ord_arc::OrdArc,
        owner::Owner,
        shared_set::{insert, SharedSet, SharedSetHandle},
    },
    Event, Mutex,
};

type ContextMutex = Mutex<Option<ContextData>>;

#[derive(Clone)]
pub struct Context {
    deadline: Option<Duration>,
    data: Arc<ContextMutex>,
}

impl Context {
    pub fn cancel(&self) {
        cancel(self.data.as_ref());
    }

    pub fn fork(&self) -> Self {
        let ctx = Context {
            deadline: self.deadline,
            data: Arc::new(Mutex::new(None)),
        };
        let parent_handle = insert(
            ContextHandle(Arc::downgrade(&self.data)),
            (&ctx.data).into(),
        );
        if parent_handle.is_some() {
            *ctx.data.lock() = Some(ContextData {
                _parent: parent_handle,
                event: Event::new(),
                children: SharedSet::new(),
            });
        }
        ctx
    }
}

struct ContextData {
    _parent: Option<SharedSetHandle<OrdArc<ContextMutex>, ContextHandle>>,
    event: Event,
    children: SharedSet<OrdArc<ContextMutex>>,
}

impl ContextData {
    fn cancel(&self) {
        self.event.notify();
        for child in self.children.iter() {
            cancel(child)
        }
    }
}

impl Drop for ContextData {
    fn drop(&mut self) {
        self.cancel()
    }
}

struct ContextHandle(Weak<ContextMutex>);

impl Owner<Event> for ContextHandle {
    fn with<U>(&self, f: impl FnOnce(&mut Event) -> U) -> Option<U> {
        Some(f(&mut self.0.upgrade()?.as_ref().lock().as_mut()?.event))
    }
}

impl Owner<SharedSet<OrdArc<ContextMutex>>> for ContextHandle {
    fn with<U>(&self, f: impl FnOnce(&mut SharedSet<OrdArc<ContextMutex>>) -> U) -> Option<U> {
        Some(f(&mut self.0.upgrade()?.as_ref().lock().as_mut()?.children))
    }
}

fn cancel(m: &Mutex<Option<ContextData>>) {
    if let Some(d) = m.lock().take() {
        d.cancel();
    }
}
