#![allow(dead_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub mod adv;
mod tests;

const INITIAL_CAP: usize = 16;
const MAX_LOAD_THRESHOLD: f32 = 0.75;
const MIN_LOAD_THRESHOLD: f32 = 0.25;
const MIN_CAPACITY: usize = 4;

struct KeyVal<K, V>
where
    K: std::hash::Hash + std::cmp::PartialEq,
{
    key: K,
    val: V,
}

impl<K, V> KeyVal<K, V>
where
    K: std::hash::Hash + std::cmp::PartialEq,
{
    fn to_tuple(&self) -> (&K, &V) {
        return (&self.key, &self.val);
    }
}

type ElementsVecs<K, V> = Vec<Vec<KeyVal<K, V>>>;
// A basic hashmap from int to int
pub struct HashMap<K, V>
where
    K: std::hash::Hash + std::cmp::PartialEq,
{
    n_elements: usize,
    element_vecs: ElementsVecs<K, V>,
}

impl<K, V> Default for HashMap<K, V>
where
    K: std::hash::Hash + std::cmp::PartialEq,
{
    fn default() -> Self {
        HashMap {
            n_elements: 0,
            element_vecs: HashMap::<K, V>::init_vecs(&INITIAL_CAP),
        }
    }
}

impl<K: std::hash::Hash + std::cmp::PartialEq, V> HashMap<K, V> {
    /// Returns a vector of KeyValue vector of size capacity
    fn init_vecs(capacity: &usize) -> ElementsVecs<K, V> {
        let mut element_vecs: ElementsVecs<K, V> = Vec::new();
        for _ in 0..*capacity {
            element_vecs.push(Vec::new());
        }
        element_vecs
    }

    /// Returns the current load factor of the hashmap, i.e num_items/capacity
    fn load_factor(&self) -> f32 {
        self.size() as f32 / self.capacity() as f32
    }

    /// Returns a new empty hashmap
    pub fn new() -> Self {
        HashMap::default()
    }

    /// Searches for a given key, and return an optional reference to the KeyValue pair
    fn find_mut(&mut self, key: &K) -> Option<&mut KeyVal<K, V>> {
        let index = self.hash(key);
        self.element_vecs[index].iter_mut().find(|x| x.key == *key)
    }

    ///
    fn find(&self, key: &K) -> Option<&KeyVal<K, V>> {
        let index = self.hash(key);
        self.element_vecs[index].iter().find(|x| x.key == *key)
    }

    /// Returns the hash value of the given key, as a function of the current capacity
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity()
    }

    /// Returns the current capacity
    fn capacity(&self) -> usize {
        self.element_vecs.len()
    }

    /// Returns true if the hashmap contains a pair with the given key, and false otherwise
    pub fn contains_key(&self, key: &K) -> bool {
        self.find(key).is_some()
    }

    /// Returns the number of items currently in the hashmap
    pub fn size(&self) -> usize {
        self.n_elements
    }

    /// Return the value associated with the given key, if exists
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(kv) = self.find(&key) {
            Some(&kv.val)
        } else {
            None
        }
    }
}
