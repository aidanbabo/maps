use std::borrow::Borrow;
use std::mem;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    next: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub(crate) struct LinkedList<K, V> {
    head: Option<Box<Node<K, V>>>,
}

impl<K, V> LinkedList<K, V> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<K, V> LinkedList<K, V>
where
    K: Eq,
{
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Eq,
        K: Borrow<Q>,
    {
        let mut curr_opt = self.head.as_ref();
        while let Some(ref curr) = curr_opt {
            if curr.key.borrow() == key {
                return Some((&curr.key, &curr.value));
            }
            curr_opt = curr.next.as_ref();
        }

        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Eq,
        K: Borrow<Q>,
    {
        let mut option = &mut self.head;

        while let Some(ref mut current) = option {
            if current.key.borrow() == key {
                return Some(&mut current.value);
            }

            option = &mut current.next;
        }

        None
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let mut option = &mut self.head;

        while let Some(ref mut current) = option {
            if current.key == key {
                mem::swap(&mut current.value, &mut value);
                return Some(value);
            }

            option = &mut current.next;
        }

        // We didn't find it in the list, so insert it at head
        self.head = Some(Box::new(Node {
            key,
            value,
            next: self.head.take(),
        }));

        None
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Eq,
        K: Borrow<Q>,
    {
        let head = match self.head.as_mut() {
            Some(head) if head.key.borrow() == key => {
                let mut head = self.head.take().unwrap();
                self.head = head.next.take();
                return Some((head.key, head.value));
            }
            Some(head) => head,
            None => return None,
        };

        let mut prev = head;

        // using complicated chains to avoid borrowing issues
        while prev.next.is_some() {
            if prev.next.as_ref().unwrap().key.borrow() == key {
                let mut ret = prev.next.take().unwrap();
                prev.next = ret.next.take();
                return Some((ret.key, ret.value));
            }

            prev = prev.next.as_mut().unwrap();
        }

        None
    }
}

// non-recursive definition to avoid stack overflow
impl<K, V> Drop for LinkedList<K, V> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut c) = curr.take() {
            curr = c.next.take();
        }
    }
}

pub(crate) struct IntoIter<K, V> {
    next: Option<Box<Node<K, V>>>,
}

impl<K, V> IntoIter<K, V> {
    fn new(mut linked_list: LinkedList<K, V>) -> Self {
        Self {
            next: linked_list.head.take(),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(node) => {
                self.next = node.next;
                Some((node.key, node.value))
            }
            None => None,
        }
    }
}

impl<K, V> IntoIterator for LinkedList<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
