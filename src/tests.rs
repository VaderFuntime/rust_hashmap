#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_hashmap() {
        let mut map = HashMap::new();
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
        map.put(1,2);
    }

    #[test]
    fn add_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.size(), 0);
        map.put(5, 6);
        assert_eq!(map.size(), 1);
        assert_eq!(*map.at(5).unwrap(), 6);
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
            assert_eq!(*map.at(i).unwrap(), i * 3);
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

    fn calc_actual_size<K: std::hash::Hash + std::cmp::PartialEq ,V>(map: &HashMap<K,V>) -> usize {
        let mut real_count = 0;

        for vec in &map.element_vecs {
            real_count += vec.len();
        }

        return real_count;
    }

    #[test]
    fn test_default_constructor() {
        let mut map = HashMap::new();
        
        assert_eq!(map.size(), 0);
        assert_eq!(map.capacity(), INITIAL_CAP);
        assert!(map.at(5).is_none());

        map.put(1,2);
    }
}