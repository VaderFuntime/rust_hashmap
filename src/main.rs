#![allow(dead_code)]

const INITIAL_CAP: usize = 16;
const MAX_LOAD_THRESHOLD: f32 = 0.75;
const MIN_LOAD_THRESHOLD: f32 = 0.25;

#[derive(Clone)]
struct KeyVal {
    key: i32,
    val: i32,
}

type ElementsVecs = Vec<Vec<KeyVal>>;
// A basic hashmap from int to int
struct HashMap {
    // capacity: usize,
    n_elements: usize,
    element_vecs: ElementsVecs,
}

impl Default for HashMap {
    fn default() -> Self {
        HashMap::new()
    }
}

impl HashMap {
    pub fn new() -> Self {
        HashMap {
            n_elements: 0,
            element_vecs: vec![vec![]; INITIAL_CAP],
        }
    }

    fn find_mut(&mut self, key: i32) -> Option<&mut KeyVal> {
        let index = self.hash(key);
        self.element_vecs[index].iter_mut().find(|x| x.key == key)
    }

    fn find(&self, key: i32) -> Option<&KeyVal> {
        let index = self.hash(key);
        self.element_vecs[index].iter().find(|x| x.key == key)
    }

    pub fn add(&mut self, key: i32, value: i32) {
        if let Some(kv) = self.find_mut(key) {
            kv.val = value;
            return;
        }
        let index = self.hash(key); // TODO why doesn't work withing the line below
        self.element_vecs[index].push(KeyVal { key, val: value });
        self.n_elements += 1;

        self.maybe_increase_capacity();
    }

    fn maybe_decrease_capacity(&mut self) -> bool {
        true // todo
    }

    fn maybe_increase_capacity(&mut self) -> bool {
        if self.load_factor() <= MAX_LOAD_THRESHOLD {
            return false;
        }
        self.rehash(self.capacity() * 2);
        return true;
    }

    fn rehash(&mut self, new_capacity: usize) {
        let old_element_vecs =
            std::mem::replace(&mut self.element_vecs,
                 vec![vec![]; new_capacity]);
        self.n_elements = 0;
        for vec in old_element_vecs.iter() {
            for kv in vec.iter() {
                self.add(kv.key, kv.val);
            }
        }
    }

    fn hash(&self, key: i32) -> usize {
        key.abs() as usize % self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.element_vecs.len()
    }

    pub fn size(&self) -> usize {
        self.n_elements
    }

    pub fn at(&self, key: i32) -> Option<i32> {
        if let Some(kv) = self.find(key) {
            Some(kv.val)
        } else {
            None
        }
    }

    fn load_factor(&self) -> f32 {
        self.size() as f32 / self.capacity() as f32
    }
}

fn main() {}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_hashmap() {
        let map = HashMap::new();
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
    }

    #[test]
    fn add_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.size(), 0);
        map.add(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(map.at(5).unwrap(), 6);
        assert!(map.at(4).is_none())
    }

    #[test]
    fn add_more() {
        let mut map = HashMap::new();
        for i in 0..20 {
            map.add(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        for i in 0..20 {
            assert_eq!(map.at(i).unwrap(), i * 3);
        }
    }

    #[test]
    fn load_factor_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.load_factor(), 0.0);

        map.add(1, 2);
        assert_eq!(map.load_factor(), 1.0 / INITIAL_CAP as f32);

        for i in 0..20 {
            map.add(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        assert_eq!(map.capacity(), 32);
        assert_eq!(map.load_factor(), 20 as f32 / 32.0)
    }
}
