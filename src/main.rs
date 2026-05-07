use rand::Rng;

struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut shuffled = self.reset();
        let n = shuffled.len();

        for i in 0..n {
            let j = rng.gen_range(0..n);
            shuffled.swap(i, j);
        }

        shuffled
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

struct Input {
    a: i32,
    b: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            a: 2,
            b: [3].to_vec(),
        },
        Input {
            a: 2,
            b: [1, 0].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::super_pow(input.a, input.b);
        println!("{:?}", result);
    }
}
