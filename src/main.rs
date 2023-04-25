mod utils;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct SmallestInfiniteSet {
    current: i32,
    memo: HashSet<i32>,
    queue: BinaryHeap<Reverse<i32>>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        return Self {
            current: 1,
            memo: HashSet::new(),
            queue: BinaryHeap::new(),
        };
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.queue.is_empty() {
            let result = self.current;
            self.current += 1;
            return result;
        }

        let result = self.queue.pop().unwrap().0;
        self.memo.remove(&result);
        return result;
    }

    fn add_back(&mut self, num: i32) {
        if self.current <= num {
            return;
        } else if self.memo.contains(&num) {
            return;
        }

        self.memo.insert(num);
        self.queue.push(Reverse(num))
    }
}

fn main() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    smallest_infinite_set.add_back(2);
    println!("{}", smallest_infinite_set.pop_smallest());
    println!("{}", smallest_infinite_set.pop_smallest());
    println!("{}", smallest_infinite_set.pop_smallest());
    smallest_infinite_set.add_back(1);
    println!("{}", smallest_infinite_set.pop_smallest());
    println!("{}", smallest_infinite_set.pop_smallest());
    println!("{}", smallest_infinite_set.pop_smallest());
}
