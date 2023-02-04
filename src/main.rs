#![allow(dead_code)]

const INITIAL_CAP: usize = 16;
const MAX_LOAD_THRESHOLD: f32 = 0.75;
const MIN_LOAD_THRESHOLD: f32 = 0.25;
const MIN_CAPACITY: usize = 4;

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
        HashMap {
            n_elements: 0,
            element_vecs: vec![vec![]; INITIAL_CAP],
        }
    }
}

// basic functions
impl HashMap {
    fn load_factor(&self) -> f32 {
        self.size() as f32 / self.capacity() as f32
    }

    pub fn new() -> Self {
        HashMap::default()
    }

    fn find_mut(&mut self, key: i32) -> Option<&mut KeyVal> {
        let index = self.hash(key);
        self.element_vecs[index].iter_mut().find(|x| x.key == key)
    }

    fn find(&self, key: i32) -> Option<&KeyVal> {
        let index = self.hash(key);
        self.element_vecs[index].iter().find(|x| x.key == key)
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
}

// adding, removing and rehashing
impl HashMap {
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(kv) = self.find_mut(key) {
            kv.val = value;
            return;
        }
        let index = self.hash(key); // TODO why doesn't work withing the line below
        self.element_vecs[index].push(KeyVal { key, val: value });
        self.n_elements += 1;

        self.maybe_increase_capacity();
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let vec_ind = self.hash(key);
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
        let old_element_vecs =
            std::mem::replace(&mut self.element_vecs, vec![vec![]; new_capacity]);
        self.n_elements = 0;
        for vec in old_element_vecs.iter() {
            for kv in vec.iter() {
                self.put(kv.key, kv.val);
            }
        }
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
        map.put(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(map.at(5).unwrap(), 6);
        assert!(map.at(4).is_none())
    }

    #[test]
    fn add_more() {
        let mut map = HashMap::new();
        for i in 0..20 {
            map.put(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        for i in 0..20 {
            assert_eq!(map.at(i).unwrap(), i * 3);
        }
        assert_eq!(map.size(), 20);
        for i in 0..20 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), 20);

        for i in 30..35 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), 25);
    }

    #[test]
    fn load_factor_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.load_factor(), 0.0);

        map.put(1, 2);
        assert_eq!(map.load_factor(), 1.0 / INITIAL_CAP as f32);

        for i in 0..20 {
            map.put(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        assert_eq!(map.capacity(), 32);
        assert_eq!(map.load_factor(), 20 as f32 / 32.0)
    }

    #[test]
    fn test_remove_basic() {
        let mut map = HashMap::new();
        map.put(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(map.capacity(), INITIAL_CAP);
        map.remove(5);
        assert_eq!(map.size(), 0);
        assert!(map.at(5).is_none());
        assert_eq!(map.remove(5), false);
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn capacity_increase_and_decrease() {
        let mut map = HashMap::new();
        for i in 0..13 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), 13);
        assert_eq!(map.capacity(), 32);

        for i in 13..25 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), 25);
        assert_eq!(map.capacity(), 64);

        for i in 20..25 {
            map.remove(i);
        }
        assert_eq!(map.size(), 20);
        assert_eq!(map.capacity(), 64);

        for i in 15..20 {
            map.remove(i);
        }
        assert_eq!(map.size(), 15);
        assert_eq!(map.capacity(), 32);

        for i in 7..15 {
            map.remove(i);
        }
        assert_eq!(map.size(), 7);
        assert_eq!(map.capacity(), 16);

        for i in 3..7 {
            map.remove(i);
        }
        assert_eq!(map.size(), 3);
        assert_eq!(map.capacity(), 8);

        for i in 0..3 {
            map.remove(i);
        }
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), 4);
    }


    #[test]
    fn test_size_is_real() {
        let mut map = HashMap::new();
        for i in 0..13 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 13..25 {
            map.put(i, i * 7);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 20..25 {
            map.remove(i);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 15..20 {
            map.remove(i);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 7..15 {
            map.remove(i);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 3..7 {
            map.remove(i);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 0..3 {
            map.remove(i);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

    }

    
    fn calc_actual_size(map: &HashMap) -> usize{
        let mut real_count = 0;

        for vec in &map.element_vecs{
            real_count += vec.len();
        }

        return real_count
    }

    #[test]
    fn test_default_constructor() {
        let map = HashMap::new();
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
        assert!(map.at(5).is_none());
    }
}
