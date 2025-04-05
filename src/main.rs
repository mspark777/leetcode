struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut or = 0usize;
        for num in nums.iter().cloned() {
            or |= num as usize;
        }

        let shift = nums.len() - 1;
        return (or << shift) as i32;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input { nums: vec![1, 3] },
        Input {
            nums: vec![5, 1, 6],
        },
        Input {
            nums: vec![1, 10, 3, 4, 19],
        },
    ];

    for input in inputs {
        let result = Solution::subset_xor_sum(input.nums);
        println!("{result:?}");
    }
}
