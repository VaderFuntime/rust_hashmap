#![allow(dead_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_CAP: usize = 16;
const MAX_LOAD_THRESHOLD: f32 = 0.75;
const MIN_LOAD_THRESHOLD: f32 = 0.25;
const MIN_CAPACITY: usize = 4;

struct KeyVal<K: std::hash::Hash + std::cmp::PartialEq, V> {
    key: K,
    val: V,
}

type ElementsVecs<K, V> = Vec<Vec<KeyVal<K, V>>>;
// A basic hashmap from int to int
pub struct HashMap<K: std::hash::Hash + std::cmp::PartialEq, V> {
    n_elements: usize,
    element_vecs: ElementsVecs<K, V>,
    
}

impl<K: std::hash::Hash + std::cmp::PartialEq, V> Default for HashMap<K, V> {
    fn default() -> Self {
        HashMap {
            n_elements: 0,
            element_vecs: HashMap::<K, V>::init_vecs(&INITIAL_CAP),
            
        }
    }
}

// basic functions
impl<K: std::hash::Hash + std::cmp::PartialEq, V> HashMap<K, V> {
    fn load_factor(&self) -> f32 {
        self.size() as f32 / self.capacity() as f32
    }

    pub fn new() -> Self {
        HashMap::default()
    }

    fn find_mut(&mut self, key: &K) -> Option<&mut KeyVal<K, V>> {
        let index = self.hash(&key);
        self.element_vecs[index].iter_mut().find(|x| x.key == *key)
    }

    fn find(&self, key: &K) -> Option<&KeyVal<K, V>> {
        let index = self.hash(key);
        self.element_vecs[index].iter().find(|x| x.key == *key)
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.element_vecs.len()
    }

    pub fn size(&self) -> usize {
        self.n_elements
    }

    pub fn at(&self, key: K) -> Option<&V> {
        if let Some(kv) = self.find(&key) {
            Some(&kv.val)
        } else {
            None
        }
    }
}

// adding, removing and rehashing
impl<K: std::hash::Hash + std::cmp::PartialEq, V> HashMap<K, V> {
    pub fn put(&mut self, key: K, value: V) {
        if let Some(kv) = self.find_mut(&key) {
            kv.val = value;
            return;
        }
        let index = self.hash(&key); // TODO why doesn't work withing the line below
        self.element_vecs[index].push(KeyVal { key, val: value });
        self.n_elements += 1;

        self.maybe_increase_capacity();
    }

    pub fn remove(&mut self, key: K) -> bool {
        let vec_ind = self.hash(&key);
        let vec = &mut self.element_vecs[vec_ind];

        if let Some(index) = vec.iter().position(|kv| kv.key == key) {
            vec.remove(index);
            self.n_elements -= 1;
            self.maybe_decrease_capacity();
            return true;
        }

        false
    }

    fn maybe_decrease_capacity(&mut self) -> bool {
        if self.load_factor() >= MIN_LOAD_THRESHOLD {
            return false;
        }

        self.rehash(std::cmp::max(self.capacity() / 2, MIN_CAPACITY));
        return true;
    }

    fn maybe_increase_capacity(&mut self) -> bool {
        if self.load_factor() <= MAX_LOAD_THRESHOLD {
            return false;
        }
        self.rehash(self.capacity() * 2);
        return true;
    }

    fn rehash(&mut self, new_capacity: usize) {
        if new_capacity == self.capacity() {
            return;
        }
        let old_element_vecs = std::mem::replace(
            &mut self.element_vecs,
            HashMap::<K, V>::init_vecs(&new_capacity),
        );
        self.n_elements = 0;
        for vec in old_element_vecs {
            for kv in vec.into_iter() {
                self.put(kv.key, kv.val);
            }
        }
    }

    fn init_vecs(capacity: &usize) -> ElementsVecs<K, V> {
        let mut element_vecs: Vec<Vec<KeyVal<K, V>>> = Vec::new();
        for _ in (0 as usize)..*capacity {
            element_vecs.push(Vec::new());
        }
        element_vecs
    }
}


