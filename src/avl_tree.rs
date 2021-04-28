#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct AvlTree<K, V> {
    _marker: std::marker::PhantomData<(K, V)>,
}
