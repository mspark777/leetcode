mod utils;

struct MyHashSet {
    keys: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        return Self {
            keys: vec![false; 1000001],
        };
    }

    fn add(&mut self, key: i32) {
        self.keys[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.keys[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        return self.keys[key as usize];
    }
}

fn main() {
    let mut my_hash_set = MyHashSet::new();
    my_hash_set.add(1);
    my_hash_set.add(2);
    println!("{}", my_hash_set.contains(1));
    println!("{}", my_hash_set.contains(3));
    my_hash_set.add(2);
    println!("{}", my_hash_set.contains(2));
    my_hash_set.remove(2);
    println!("{}", my_hash_set.contains(2));
}
