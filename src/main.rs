mod utils;

use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    queue: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut queue = BinaryHeap::<Reverse<i32>>::with_capacity(k + 1);
        for &num in nums.iter() {
            Self::add_val(&mut queue, num, k)
        }

        return Self { k, queue };
    }

    fn add(&mut self, val: i32) -> i32 {
        Self::add_val(&mut self.queue, val, self.k);

        return self.queue.peek().unwrap().0;
    }

    fn add_val(queue: &mut BinaryHeap<Reverse<i32>>, val: i32, size: usize) {
        queue.push(Reverse(val));

        if queue.len() > size {
            queue.pop();
        }
    }
}

fn main() {
    let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", kth_largest.add(3));
    println!("{}", kth_largest.add(5));
    println!("{}", kth_largest.add(10));
    println!("{}", kth_largest.add(9));
    println!("{}", kth_largest.add(4));
}
