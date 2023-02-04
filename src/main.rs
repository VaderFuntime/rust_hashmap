const INITAL_CAP: i32 = 16;

struct KeyVal {
    key: i32,
    val: i32,
}

// A basic hashmap from int to int
struct MyHashMap {
    // capacity: usize,
    size: usize,
    items: Vec<Vec<KeyVal>>,
}

impl MyHashMap {
    fn get_capacity(&self) -> usize {
        self.items.capacity()
    }

    fn find(&mut self, key: i32) -> Option<&mut KeyVal> {
        let index = self.get_hash(key);
        self.items[index].iter_mut().find(|x| x.key == key)
    }

    fn add(&mut self, key: i32, value: i32) {
        if let Some(kv) = self.find(key){
            kv.val = value;
            return;
        }
        let index = self.get_hash(key); // TODO bla bla
        self.items[index].push(KeyVal { key, val: value });
    }

    fn get_hash(&self, item: i32) -> usize {
        item.abs() as usize % self.items.capacity()
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
