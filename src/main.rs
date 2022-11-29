use rand::random;
use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    indexes: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        return Self {
            nums: vec![],
            indexes: HashMap::new(),
        };
    }

    fn insert(&mut self, val: i32) -> bool {
        let indexes = &mut self.indexes;
        if indexes.contains_key(&val) {
            return false;
        }

        let nums = &mut self.nums;
        indexes.insert(val, nums.len());
        nums.push(val);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        let indexes = &mut self.indexes;
        if !indexes.contains_key(&val) {
            return false;
        }

        let nums = &mut self.nums;
        let last = *nums.last().unwrap();
        let pos = *indexes.get(&val).unwrap();

        *indexes.get_mut(&last).unwrap() = pos;
        nums[pos] = last;

        indexes.remove(&val);
        nums.pop();
        return true;
    }

    fn get_random(&self) -> i32 {
        let nums = &self.nums;
        let idx = random::<usize>() % nums.len();

        return nums[idx];
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    println!("{}", obj.insert(3));
    println!("{}", obj.insert(3));
    println!("{}", obj.get_random());
    println!("{}", obj.get_random());
    println!("{}", obj.insert(1));
    println!("{}", obj.remove(3));
    println!("{}", obj.get_random());
    println!("{}", obj.insert(0));
    println!("{}", obj.remove(0));
}
