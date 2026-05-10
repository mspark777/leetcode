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

    fn pick(&self, target: i32) -> i32 {
        let mut idx = 0usize;
        let mut count = 0;
        for (i, num) in self.nums.iter().copied().enumerate() {
            if num != target {
                continue;
            }

            count += 1;

            if (rand::random::<i32>() % count) == 0 {
                idx = i;
            }
        }

        idx as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 8 }, Input { n: 7 }];

    for input in inputs.into_iter() {
        let result = Solution::integer_replacement(input.n);
        println!("{:?}", result);
    }
}
