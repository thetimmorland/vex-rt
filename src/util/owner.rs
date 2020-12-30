use core::marker::PhantomData;

pub trait Owner<T> {
    fn with<U>(&self, f: impl FnOnce(&mut T) -> U) -> Option<U>;
}

#[allow(dead_code)]
pub fn map<'a, T: 'a, U: 'a>(
    owner: &'a impl Owner<T>,
    f: impl 'a + Fn(&mut T) -> &mut U,
) -> impl Owner<U> + 'a {
    struct OwnerWrapper<'a, T, U, O, F: Fn(&mut T) -> &mut U>(&'a O, F, PhantomData<T>);
    impl<'a, T, U, O: Owner<T>, F: Fn(&mut T) -> &mut U> Owner<U> for OwnerWrapper<'a, T, U, O, F> {
        fn with<V>(&self, f: impl FnOnce(&mut U) -> V) -> Option<V> {
            self.0.with(|t| f(self.1(t)))
        }
    }

    OwnerWrapper(owner, f, PhantomData)
}
