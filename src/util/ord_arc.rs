use alloc::sync::Arc;
use core::ops::Deref;

pub struct OrdArc<T: ?Sized>(Arc<T>);

impl<T> OrdArc<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl<T: ?Sized> From<&Arc<T>> for OrdArc<T> {
    fn from(arc: &Arc<T>) -> Self {
        Self(arc.clone())
    }
}

impl<T: ?Sized> Deref for OrdArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T: ?Sized> Clone for OrdArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for OrdArc<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl<T: ?Sized> Eq for OrdArc<T> {}

impl<T: ?Sized> PartialOrd for OrdArc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Arc::as_ptr(&self.0).partial_cmp(&Arc::as_ptr(&other.0))
    }
}

impl<T: ?Sized> Ord for OrdArc<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Arc::as_ptr(&self.0).cmp(&Arc::as_ptr(&other.0))
    }
}
