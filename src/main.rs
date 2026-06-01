use rand::prelude::*;

struct Solution {
    rows: i32,
    cols: i32,
    rng: ThreadRng,
    flipped: std::collections::HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            rows: m,
            cols: n,
            rng: rand::thread_rng(),
            flipped: std::collections::HashSet::new(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut random: i32 = self.rng.gen_range(0..(self.rows * self.cols));

        while self.flipped.contains(&random) {
            random += 1;
            random = random % (self.rows * self.cols);
        }

        self.flipped.insert(random);

        vec![random / self.cols, random % self.cols]
    }

    fn reset(&mut self) {
        self.flipped.clear();
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 3].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::next_greater_elements(input.nums);
        println!("{:?}", result);
    }
}
