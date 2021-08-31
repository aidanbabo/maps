use std::borrow::Borrow;
use std::mem;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<K, V> {
    hash: u64,
    key: K,
    value: V,
    left: Option<NonNull<Node<K, V>>>,
    right: Option<NonNull<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(hash: u64, key: K, value: V) -> Self {
        Self {
            hash,
            key,
            value,
            right: None,
            left: None,
        }
    }
}
impl<K, V> Node<K, V>
where
    K: Eq,
{
    fn insert(&mut self, hash: u64, key: K, value: V) -> Option<V> {
        if self.hash == hash && self.key == key {
            let mut value = value;
            mem::swap(&mut self.value, &mut value);
            Some(value)
        } else if hash < self.hash {
            if let Some(ref mut left) = self.left {
                // TODO rebalancing check
                // must always be init
                unsafe { left.as_mut() }.insert(hash, key, value)
            } else {
                self.left = unsafe {
                    Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(
                        hash, key, value,
                    )))))
                };
                None
            }
        } else {
            if let Some(ref mut right) = self.right {
                // TODO rebalancing check
                // must always be init
                unsafe { right.as_mut() }.insert(hash, key, value)
            } else {
                self.right = unsafe {
                    Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(
                        hash, key, value,
                    )))))
                };
                None
            }
        }
    }

    fn get_key_value<Q: ?Sized>(&self, hash: u64, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if self.hash == hash && self.key.borrow() == key {
            Some((&self.key, &self.value))
        } else if hash < self.hash {
            if let Some(ref left) = self.left {
                // must always be init
                unsafe { left.as_ref() }.get_key_value(hash, key)
            } else {
                None
            }
        } else {
            if let Some(ref right) = self.right {
                // must always be init
                unsafe { right.as_ref() }.get_key_value(hash, key)
            } else {
                None
            }
        }
    }

    fn get_mut<Q: ?Sized>(&mut self, hash: u64, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if self.hash == hash && self.key.borrow() == key {
            Some(&mut self.value)
        } else if hash < self.hash {
            if let Some(ref mut left) = self.left {
                unsafe { left.as_mut() }.get_mut(hash, key)
            } else {
                None
            }
        } else {
            if let Some(ref mut right) = self.right {
                // must always be init
                unsafe { right.as_mut() }.get_mut(hash, key)
            } else {
                None
            }
        }
    }

    // TODO ahhh
    fn remove_entry<Q: ?Sized>(&mut self, hash: u64, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if self.hash == hash && self.key.borrow() == key {
        } else if hash < self.hash {
        } else {
        }
        return None;
    }

    fn find_leftmost(&mut self) -> Option<NonNull<Node<K, V>>> {
        None
    }
}

#[derive(Debug)]
/// Objects with equal hash will always be put to the right
pub(crate) struct AvlTree<K, V> {
    root: Option<Node<K, V>>,
}

impl<K, V> AvlTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<K, V> AvlTree<K, V>
where
    K: Eq,
{
    pub fn insert(&mut self, hash: u64, key: K, value: V) -> Option<V> {
        if let Some(ref mut root) = self.root {
            root.insert(hash, key, value)
        } else {
            self.root = Some(Node::new(hash, key, value));
            None
        }
    }

    pub fn get_key_value<Q: ?Sized>(&self, hash: u64, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if let Some(ref root) = self.root {
            root.get_key_value(hash, key)
        } else {
            None
        }
    }

    pub fn get_mut<Q: ?Sized>(&mut self, hash: u64, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if let Some(ref mut root) = self.root {
            root.get_mut(hash, key)
        } else {
            None
        }
    }

    // TODO ahhh
    pub fn remove_entry<Q: ?Sized>(&mut self, hash: u64, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if let Some(ref mut root) = self.root {
            root.remove_entry(hash, key)
        } else {
            None
        }
    }
}

pub(crate) struct IntoIter<K, V> {
    lineage: Vec<Node<K, V>>,
}

fn add_left<K, V>(to: &mut Vec<Node<K, V>>, from: Option<NonNull<Node<K, V>>>) {
    let mut node = from;
    loop {
        if let Some(left) = node {
            let mut left = unsafe { Box::from_raw(left.as_ptr()) };
            let new = left.left.take();
            to.push(*left);
            node = new;
        } else {
            break;
        }
    }
}

impl<K, V> IntoIter<K, V> {
    fn new(tree: AvlTree<K, V>) -> Self {
        let mut lineage = Vec::new();
        if let Some(mut root) = tree.root {
            let left = root.left.take();
            lineage.push(root);
            add_left(&mut lineage, left);
        }
        Self { lineage }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut next) = self.lineage.pop() {
            add_left(&mut self.lineage, next.right.take());
            Some((next.key, next.value))
        } else {
            None
        }
    }
}

impl<K, V> IntoIterator for AvlTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
