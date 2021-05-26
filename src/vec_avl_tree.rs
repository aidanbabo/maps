use std::borrow::Borrow;
use std::mem;

#[derive(Debug)]
/// Objects with equal hash will always be put to the right
pub(crate) struct VecAvlTree<K, V> {
    buf: Vec<Option<Entry<K, V>>>,
}

impl<K, V> AvlTree<K, V> {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl<K, V> AvlTree<K, V>
where
    K: Eq,
{
    pub fn insert(&mut self, hash: usize, key: K, mut value: V) -> Option<V> {
        // handle special case at root of tree
        let mut left_next = match self.entry_mut(self.root()) {
            Some(entry) => match entry.hash {
                h if h == hash && entry.key == key => {
                    mem::swap(&mut entry.value, &mut value);
                    return Some(value);
                }
                h if h < hash => true,
                _ => false,
            },
            None => {
                // wrap?
                self.buf.push(Entry { hash, key, value });
                return None;
            }
        };

        let mut prev = 0;

        loop {
            // next node to work with, if it's empty, insert our element
            let next = if left_next {
                if self.entry(self.left(prev)).is_none() {
                    prev.left = Some(Box::new(Node {
                        hash,
                        key,
                        value,
                        left: None,
                        right: None,
                    }));
                    return None;
                } else {
                    prev.left.as_mut().unwrap()
                }
            } else {
                if self.entry(self.left(prev)).is_none() {
                    prev.right = Some(Box::new(Node {
                        hash,
                        key,
                        value,
                        left: None,
                        right: None,
                    }));
                    return None;
                } else {
                    prev.right.as_mut().unwrap()
                }
            };

            // move to the next node
            match next.hash {
                h if h == hash && next.key == key => {
                    mem::swap(&mut next.value, &mut value);
                    return Some(value);
                }
                h if h < hash => {
                    left_next = true;
                    prev = next;
                }
                _ => {
                    left_next = false;
                    prev = next;
                }
            }
        }
    }

    pub fn get_key_value<Q: ?Sized>(&self, hash: usize, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        let mut node = &self.root;

        while let Some(n) = node {
            match n.hash {
                h if h == hash && n.key.borrow() == key => {
                    return Some((&n.key, &n.value));
                }
                h if h < hash => node = &n.left,
                _ => node = &n.right,
            }
        }

        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, hash: usize, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        let mut node = &mut self.root;

        while let Some(n) = node {
            match n.hash {
                h if h == hash && n.key.borrow() == key => {
                    return Some(&mut n.value);
                }
                h if h < hash => node = &mut n.left,
                _ => node = &mut n.right,
            }
        }

        None
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, hash: usize, key: K) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        None
    }

    fn extract_successor(of_node: &mut Node<K, V>) -> Option<Box<Node<K, V>>> {
        let mut start = if let Some(ref mut right) = of_node.right {
            right
        } else {
            return None;
        };

        while let Some(ref mut left) = start.left {
            start = left;
        }

        start.left.take()
    }
}
