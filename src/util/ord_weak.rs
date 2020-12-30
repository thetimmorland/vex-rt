use alloc::sync::{Arc, Weak};

pub struct OrdWeak<T: ?Sized>(Weak<T>);

impl<T> OrdWeak<T> {
    pub fn upgrade(&self) -> Option<Arc<T>> {
        self.0.upgrade()
    }
}

impl<T: ?Sized> From<Weak<T>> for OrdWeak<T> {
    fn from(weak: Weak<T>) -> Self {
        Self(weak.clone())
    }
}

impl<T: ?Sized> Clone for OrdWeak<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for OrdWeak<T> {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}

impl<T: ?Sized> Eq for OrdWeak<T> {}

impl<T: ?Sized> PartialOrd for OrdWeak<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Weak::as_ptr(&self.0).partial_cmp(&Weak::as_ptr(&other.0))
    }
}

impl<T: ?Sized> Ord for OrdWeak<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Weak::as_ptr(&self.0).cmp(&Weak::as_ptr(&other.0))
    }
}
