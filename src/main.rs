struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let n = n as usize;
        let mut nums = vec![0; n + 1];
        let mut result = 1;

        nums[1] = 1;
        for i in 2..nums.len() {
            if (i & 1) == 1 {
                let idx = i / 2;
                nums[i] = nums[idx] + nums[idx + 1];
            } else {
                nums[i] = nums[i / 2];
            }
            result = result.max(nums[i]);
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 7 }, Input { n: 2 }, Input { n: 3 }];

    for input in inputs {
        let result = Solution::get_maximum_generated(input.n);
        println!("{:?}", result);
    }
}
