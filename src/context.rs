use alloc::sync::{Arc, Weak};
use core::{cmp::min, time::Duration};

use crate::{
    handle_event, time_since_start,
    util::{
        ord_weak::OrdWeak,
        owner::Owner,
        shared_set::{insert, SharedSet, SharedSetHandle},
    },
    Event, EventHandle, GenericSleep, Mutex, Selectable,
};

type ContextMutex = Mutex<Option<ContextData>>;

#[derive(Clone)]
pub struct Context {
    deadline: Option<Duration>,
    data: Arc<ContextMutex>,
}

impl Context {
    pub fn new_global() -> Self {
        Self {
            deadline: None,
            data: Arc::new(Mutex::new(Some(ContextData {
                _parent: None,
                event: Event::new(),
                children: SharedSet::new(),
            }))),
        }
    }

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
            Arc::downgrade(&ctx.data).into(),
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

    pub fn fork_with_deadline(&self, deadline: Duration) -> Self {
        let mut ctx = self.fork();
        ctx.deadline = Some(ctx.deadline.map_or(deadline, |d| min(d, deadline)));
        ctx
    }

    pub fn fork_with_timeout(&self, timeout: Duration) -> Self {
        self.fork_with_deadline(time_since_start() + timeout)
    }

    pub fn done<'a>(&'a self) -> impl Selectable + 'a {
        struct ContextSelect<'a>(&'a Context, EventHandle<ContextHandle>);

        impl<'a> Selectable for ContextSelect<'a> {
            fn poll(self) -> Result<(), Self> {
                let mut lock = self.0.data.lock();
                let opt = &mut lock.as_mut();
                if opt.is_some() {
                    if self.0.deadline.map_or(false, |v| v <= time_since_start()) {
                        opt.take();
                        Ok(())
                    } else {
                        Err(self)
                    }
                } else {
                    Ok(())
                }
            }
            fn sleep(&self) -> GenericSleep {
                GenericSleep::NotifyTake(self.0.deadline)
            }
        }

        ContextSelect(
            self,
            handle_event(ContextHandle(Arc::downgrade(&self.data))),
        )
    }
}

struct ContextData {
    _parent: Option<SharedSetHandle<OrdWeak<ContextMutex>, ContextHandle>>,
    event: Event,
    children: SharedSet<OrdWeak<ContextMutex>>,
}

impl Drop for ContextData {
    fn drop(&mut self) {
        self.event.notify();
        for child in self.children.iter() {
            child.upgrade().map(|c| cancel(&c));
        }
    }
}

struct ContextHandle(Weak<ContextMutex>);

impl Owner<Event> for ContextHandle {
    fn with<U>(&self, f: impl FnOnce(&mut Event) -> U) -> Option<U> {
        Some(f(&mut self.0.upgrade()?.as_ref().lock().as_mut()?.event))
    }
}

impl Owner<SharedSet<OrdWeak<ContextMutex>>> for ContextHandle {
    fn with<U>(&self, f: impl FnOnce(&mut SharedSet<OrdWeak<ContextMutex>>) -> U) -> Option<U> {
        Some(f(&mut self.0.upgrade()?.as_ref().lock().as_mut()?.children))
    }
}

fn cancel(m: &Mutex<Option<ContextData>>) {
    m.lock().take();
}
