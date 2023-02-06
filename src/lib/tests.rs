

#[cfg(test)]
mod tests {
    use super::super::*;
    
    #[test]
    fn new_hashmap() {
        let mut map = HashMap::new();
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
        map.insert(1, 2);
    }

    #[test]
    fn insert_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.size(), 0);
        map.insert(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(*map.get(&5).unwrap(), 6);
        assert!(map.get(&4).is_none())
    }

    #[test]
    fn insert_more() {
        let mut map = HashMap::new();
        for i in 0..20 {
            map.insert(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        for i in 0..20 {
            assert_eq!(*map.get(&i).unwrap(), i * 3);
        }
        assert_eq!(map.size(), 20);
        for i in 0..20 {
            map.insert(i, i * 7);
        }
        assert_eq!(map.size(), 20);

        for i in 30..35 {
            map.insert(i, i * 7);
        }
        assert_eq!(map.size(), 25);
    }

    #[test]
    fn test_contains_key() {
        let mut map = HashMap::new();
        assert!(!map.contains_key(&5));

        map.insert(5, 6);
        assert!(map.contains_key(&5));
    }

    #[test]
    fn test_insert_or() {
        let mut map = HashMap::new();
        map.insert(5, 6);
        assert_eq!(map.get(&5), Some(&6));
        map.insert(5, 7);
        assert_eq!(map.get(&5), Some(&7));

        map.weak_insert(5, 8);
        assert_eq!(map.get(&5), Some(&7));
    }

    #[test]
    fn test_string_key() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 4);
        assert!(map.contains_key(&String::from("key")));
        assert_eq!(map.get(&String::from("key")), Some(&4));
    }
    
    #[test]
    fn test_string_key_val() {
        let mut map = HashMap::new();
        map.insert("key".to_string(),  "val".to_string());
        assert!(map.contains_key(&String::from("key")));
        assert_eq!(map.get(&String::from("key")), Some(&"val".to_string()));
    }

    #[test]
    fn load_factor_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.load_factor(), 0.0);

        map.insert(1, 2);
        assert_eq!(map.load_factor(), 1.0 / INITIAL_CAP as f32);

        for i in 0..20 {
            map.insert(i, i * 3);
        }
        assert_eq!(map.size(), 20);
        assert_eq!(map.capacity(), 32);
        assert_eq!(map.load_factor(), 20_f32 / 32.0)
    }

    #[test]
    fn test_remove_basic() {
        let mut map = HashMap::new();
        map.insert(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(map.capacity(), INITIAL_CAP);
        map.remove(5);
        assert_eq!(map.size(), 0);
        assert!(map.get(&5).is_none());
        assert!(!map.remove(5));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn capacity_increase_and_decrease() {
        let mut map = HashMap::new();
        for i in 0..13 {
            map.insert(i, i * 7);
        }
        assert_eq!(map.size(), 13);
        assert_eq!(map.capacity(), 32);

        for i in 13..25 {
            map.insert(i, i * 7);
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
            map.insert(i, i * 7);
        }
        assert_eq!(map.size(), calc_actual_size(&map));

        for i in 13..25 {
            map.insert(i, i * 7);
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

    fn calc_actual_size<K: std::hash::Hash + std::cmp::PartialEq, V>(map: &HashMap<K, V>) -> usize {
        let mut real_count = 0;

        for vec in &map.element_vecs {
            real_count += vec.len();
        }

        real_count
    }

    #[test]
    fn test_default_constructor() {
        let mut map = HashMap::new();

        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
        assert!(map.get(&5).is_none());

        map.insert(1, 2);
    }
}