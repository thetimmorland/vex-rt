use alloc::collections::{btree_set::Iter, BTreeSet};

use crate::util::owner::Owner;

pub struct SharedSet<T: Ord + Clone>(BTreeSet<T>);

impl<T: Ord + Clone> SharedSet<T> {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }
}

pub fn insert<T: Ord + Clone, O: Owner<SharedSet<T>>>(
    owner: O,
    value: T,
) -> Option<SharedSetHandle<T, O>> {
    owner.with(|set| set.0.insert(value.clone()))?;
    Some(SharedSetHandle { owner, value })
}

pub struct SharedSetHandle<T: Ord + Clone, O: Owner<SharedSet<T>>> {
    owner: O,
    value: T,
}

impl<T: Ord + Clone, O: Owner<SharedSet<T>>> Drop for SharedSetHandle<T, O> {
    fn drop(&mut self) {
        self.owner.with(|set| set.0.remove(&self.value));
    }
}
