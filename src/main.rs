use rand::Rng;
use std::collections::BTreeMap;

struct Solution {
    btm: BTreeMap<i32, i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
        let mut n = 0;
        for (i, weight) in w.into_iter().enumerate() {
            n += weight;
            btm.insert(n, i as i32);
        }
        Self { btm, n }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let m: i32 = rng.gen_range(0..self.n);
        self.btm
            .range(m + 1..)
            .next()
            .map(|v| v.1)
            .copied()
            .unwrap_or(-1)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 2 }, Input { n: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::count_arrangement(input.n);
        println!("{:?}", result);
    }
}
