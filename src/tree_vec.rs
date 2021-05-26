#[derive(Debug)]
pub(crate) struct Entry<K, V> {
    hash: usize,
    key: K,
    value: V,
}

#[derive(Debug)]
pub(crate) struct TreeAddr(Option<usize>);

impl TreeAddr {}

#[derive(Debug)]
pub(crate) struct TreeVec<K, V>(Vec<Option<Entry<K, V>>>);

impl<K, V> TreeVec<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, addr: TreeAddr) -> bool {
        addr.0.is_none()
    }

    pub fn root(&self) -> TreeAddr {
        TreeAddr((!self.0.is_empty()).then(|| 0))
    }

    pub fn parent(&self, addr: TreeAddr) -> TreeAddr {
        TreeAddr(addr.0.filter(|&i| i != 0).map(|i| i / 2))
    }

    pub fn left_child(&self, addr: TreeAddr) -> TreeAddr {
        TreeAddr(addr.0.map(|i| 2 * i).filter(|&i| self.0.get(i).is_some()))
    }
    pub fn right_child(&self, addr: TreeAddr) -> TreeAddr {
        TreeAddr(
            addr.0
                .map(|i| 2 * i + 1)
                .filter(|&i| self.0.get(i).is_some()),
        )
    }
}
