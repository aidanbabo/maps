use std::borrow::Borrow;
use std::mem;

#[derive(Debug)]
struct Node<K, V> {
    hash: u64,
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
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
                left.insert(hash, key, value)
            } else {
                self.left = Some(Box::new(Node::new(hash, key, value)));
                None
            }
        } else {
            if let Some(ref mut right) = self.right {
                // TODO rebalancing check
                right.insert(hash, key, value)
            } else {
                self.right = Some(Box::new(Node::new(hash, key, value)));
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
                left.get_key_value(hash, key)
            } else {
                None
            }
        } else {
            if let Some(ref right) = self.right {
                right.get_key_value(hash, key)
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
                left.get_mut(hash, key)
            } else {
                None
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.get_mut(hash, key)
            } else {
                None
            }
        }
    }

    // TODO ahhh
    fn remove_entry<Q: ?Sized>(&mut self, _hash: u64, _key: &Q) -> (Option<(K, V)>, bool)
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        (None, false)
        /*
        if self.hash == hash && self.key.borrow() == key {
            let left = self.left.take();
            let mut right = self.right.take();

            let succ = {
                if let Some(ref mut right) = right {
                    right.find_leftmost()
                } else {
                    None
                }
            };
            if let Some(mut succ) = succ {
                mem::swap(self, &mut succ);
                (Some((succ.key, succ.value)), false)
            } else {
                // We need to somehow set self to be None
                let Node { key, value, .. } = *self;
                (Some((key, value)), true)
            }
        } else if hash < self.hash {
            if let Some(ref mut left) = self.left {
                match left.remove_entry(hash, key) {
                    (ret, true) => {
                        self.left = None;
                        (ret, false)
                    }
                    good_to_forward => good_to_forward,
                }
            } else {
                (None, false)
            }
        } else {
            if let Some(ref mut right) = self.right {
                match right.remove_entry(hash, key) {
                    (ret, true) => {
                        self.right = None;
                        (ret, false)
                    }
                    good_to_forward => good_to_forward,
                }
            } else {
                (None, false)
            }
        }
        */
    }

    /*
    fn find_leftmost(&mut self) -> Option<Box<Node<K, V>>> {
        None
    }
    */
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
            match root.remove_entry(hash, key) {
                (ret, true) => {
                    self.root = None;
                    ret
                }
                (ret, _) => ret,
            }
        } else {
            None
        }
    }
}

pub(crate) struct IntoIter<K, V> {
    lineage: Vec<Node<K, V>>,
}

fn add_left<K, V>(to: &mut Vec<Node<K, V>>, from: Option<Box<Node<K, V>>>) {
    let mut node = from;
    loop {
        if let Some(mut left) = node {
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
