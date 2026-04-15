struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }

        let exclude_first = Self::start(&nums, 1, n - 1);
        let exclude_last = Self::start(&nums, 0, n - 2);

        exclude_first.max(exclude_last)
    }

    fn start(nums: &[i32], left: usize, right: usize) -> i32 {
        let mut prev1 = 0;
        let mut prev2 = 0;

        for num in nums.iter().skip(left).take(right + 1).copied() {
            let curr = prev1.max(prev2 + num);
            prev2 = prev1;
            prev1 = curr;
        }

        prev1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 3, 2].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::rob(input.nums);
        println!("{:?}", result);
    }
}
