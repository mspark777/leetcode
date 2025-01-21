struct Solution {}

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for n in nums.iter().skip(1).cloned() {
            min = min.min(n);
            max = max.max(n);
        }

        return 0.max(max - min - 2 * k);
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1],
            k: 0,
        },
        Input {
            nums: vec![0, 10],
            k: 2,
        },
        Input {
            nums: vec![1, 3, 6],
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::smallest_range_i(input.nums, input.k);
        println!("{result:?}");
    }
}
