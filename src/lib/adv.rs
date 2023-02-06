#![allow(dead_code)]

use super::*;

impl<K, V> HashMap<K, V>
where
    K: std::hash::Hash + std::cmp::PartialEq,
{
    /// Inserts the key value pair to the hashmap
    /// If a pair with this key already exists, overrides the old value
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(kv) = self.find_mut(&key) {
            kv.val = value;
            return;
        }
        let index = self.hash(&key); // TODO why doesn't work withing the line below
        self.element_vecs[index].push(KeyVal { key, val: value });
        self.n_elements += 1;

        self.maybe_increase_capacity();
    }

    /// Inserts the key value pair, but doesn't override the old value
    /// if a pair already exists    
    pub fn weak_insert(&mut self, key: K, value: V) {
        if self.contains_key(&key) {
            return;
        }
        self.insert(key, value);
    }

    /// Removes the pair associated with 'key'
    /// Returns true if was removed, and false if a pair with this key does not exist
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

    /// Checks whether capacity needs to be decreased, according to the current load factor
    /// If so, decreases the capacity and re-hashes all pairs
    fn maybe_decrease_capacity(&mut self) -> bool {
        if self.load_factor() >= MIN_LOAD_THRESHOLD {
            return false;
        }

        self.rehash(std::cmp::max(self.capacity() / 2, MIN_CAPACITY));
        true
    }

    /// Checks whether capacity needs to be increased, according to the current load factor
    /// If so, increases the capacity and re-hashes all pairs
    fn maybe_increase_capacity(&mut self) -> bool {
        if self.load_factor() <= MAX_LOAD_THRESHOLD {
            return false;
        }
        self.rehash(self.capacity() * 2);
        true
    }

    /// Re-hashes all KeyValue pairs to new vectors, with the given 'new_capacity'
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
                self.insert(kv.key, kv.val);
            }
        }
    }
}
