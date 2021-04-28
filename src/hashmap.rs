use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::FromIterator;

use crate::avl_tree::AvlTree;
use crate::linked_list::LinkedList;

#[derive(Debug)]
enum Entry<K, V> {
    ListEntry(LinkedList<K, V>),
    #[allow(dead_code)]
    TreeEntry(AvlTree<K, V>),
    Empty,
}

impl<K, V> Default for Entry<K, V> {
    fn default() -> Self {
        Entry::Empty
    }
}

const LOAD_FACTOR: f64 = 0.75;
const DEFAULT_CAPACITY: usize = 16;

#[derive(Debug)]
pub struct HashMap<K, V, S = RandomState> {
    table: Box<[Entry<K, V>]>,
    hash_builder: S,
    len: usize,
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> Self {
        Self::with_capacity_and_hasher(DEFAULT_CAPACITY, RandomState::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_and_hasher(cap, RandomState::new())
    }
}

impl<K, V, S> HashMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self::with_capacity_and_hasher(DEFAULT_CAPACITY, hash_builder)
    }

    // TODO resizing guarantees
    pub fn with_capacity_and_hasher(cap: usize, hash_builder: S) -> Self {
        let mut capacity = 1;
        while capacity < cap {
            capacity <<= 1;
        }

        let mut v = Vec::new();
        for _ in 0..capacity {
            v.push(Default::default());
        }
        let table = v.into_boxed_slice();

        Self {
            table,
            hash_builder,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn hash_index<Q: ?Sized>(&self, key: &Q) -> usize
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        let mut hasher = self.hash_builder.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        hash & (self.table.len() - 1)
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.get_key_value(key).is_some()
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        let index = self.hash_index(key);

        match &mut self.table[index] {
            Entry::ListEntry(list) => list.get_mut(key),
            Entry::TreeEntry(_list) => unimplemented!(),
            Entry::Empty => None,
        }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.get_key_value(key).map(|(_k, v)| v)
    }

    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        let index = self.hash_index(key);

        match &self.table[index] {
            Entry::ListEntry(list) => list.get_key_value(key),
            Entry::TreeEntry(_list) => unimplemented!(),
            Entry::Empty => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_index(&key);

        let ret = Self::insert_into_table(&mut self.table, index, key, value);
        if ret.is_none() {
            self.len += 1;
        }

        if self.len() >= (LOAD_FACTOR * self.table.len() as f64) as usize {
            self.resize();
        }

        ret
    }

    fn insert_into_table(table: &mut [Entry<K, V>], index: usize, key: K, value: V) -> Option<V> {
        match &mut table[index] {
            Entry::ListEntry(list) => {
                let res = list.insert(key, value);
                res
            }
            Entry::TreeEntry(_list) => unimplemented!(),
            Entry::Empty => {
                let mut entry = LinkedList::new();
                entry.insert(key, value);
                table[index] = Entry::ListEntry(entry);
                None
            }
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.remove_entry(key).map(|(_k, v)| v)
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        let index = self.hash_index(key);

        match &mut self.table[index] {
            Entry::ListEntry(list) => {
                let res = list.remove_entry(key);
                if res.is_some() {
                    self.len -= 1;
                }
                if list.is_empty() {
                    self.table[index] = Entry::Empty;
                }
                res
            }

            Entry::TreeEntry(_list) => unimplemented!(),
            Entry::Empty => None,
        }
    }

    fn resize(&mut self) {
        // new capacity is twice as large
        let new_cap = self.table.len() << 1;

        let mut v = Vec::new();
        for _ in 0..new_cap {
            v.push(Default::default());
        }

        // Swap in new table size
        let mut old_table = v.into_boxed_slice();
        std::mem::swap(&mut self.table, &mut old_table);

        // by value iterator
        for entry in Vec::from(old_table) {
            match entry {
                Entry::ListEntry(list) => {
                    for (k, v) in list {
                        let index = self.hash_index(&k);

                        // ignores resizing
                        Self::insert_into_table(&mut self.table, index, k, v);
                    }
                }
                Entry::TreeEntry(_tree) => unimplemented!(),
                Entry::Empty => {}
            }
        }
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for HashMap<K, V> {
    // TODO: use sizehint?
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = HashMap::new();

        for (k, v) in iter {
            map.insert(k, v);
        }

        map
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_len() {
        let map: HashMap<(), ()> = HashMap::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn get_non_existent_key() {
        let map: HashMap<(), ()> = HashMap::new();
        assert_eq!(map.get(&()), None);
    }

    #[test]
    fn insert_one() {
        let mut map = HashMap::new();
        assert_eq!(map.insert(1, 2), None);
        println!("{:?}", map.table);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_and_replace_one() {
        let mut map = HashMap::new();
        assert_eq!(map.insert(1, 2), None);
        println!("{:?}", map.table);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.insert(1, 3), Some(2));
        println!("{:?}", map.table);
        assert_eq!(map.get(&1), Some(&3));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_many() {
        let mut map = HashMap::new();
        for i in 0..1000 {
            assert_eq!(map.insert(i, i + 1), None);
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }
    }

    #[test]
    fn insert_and_replace_many() {
        let mut map = HashMap::new();
        for i in 0..1000 {
            assert_eq!(map.insert(i, i + 1), None);
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }

        for i in 0..1000 {
            assert_eq!(map.insert(i, i + i + 1), Some(i + 1));
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i + i + 1)));
        }
    }

    #[test]
    fn insert_and_remove_one() {
        let mut map = HashMap::new();
        assert_eq!(map.insert(1, 2), None);
        println!("{:?}", map.table);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&1), Some(2));
        println!("{:?}", map.table);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert_and_remove_many() {
        let mut map = HashMap::new();
        for i in 0..1000 {
            assert_eq!(map.insert(i, i + 1), None);
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }

        for i in 0..1000 {
            assert_eq!(map.remove(&i), Some(i + 1));
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), None);
        }
    }

    #[test]
    fn from_iter() {
        let map: HashMap<_, _> = (0..1000).map(|i| (i, i + 1)).collect();

        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }
    }
}
