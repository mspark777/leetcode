struct Solution;

impl Solution {
    pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut result = 0;

        for (left, right) in nums
            .iter()
            .copied()
            .zip(nums.iter().copied().rev())
            .take(k as usize)
        {
            result += right - left;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [5, 2, 2, 4].to_vec(),
            k: 2,
        },
        Input {
            nums: [100].to_vec(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::abs_difference(input.nums, input.k);
        println!("{:?}", result);
    }
}
