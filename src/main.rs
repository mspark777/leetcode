struct Solution {}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut min_prefix_sum = 0;
        let mut max_prefix_sum = 0;
        let mut prefix_sum = 0;

        for num in nums.iter().cloned() {
            prefix_sum += num;

            min_prefix_sum = min_prefix_sum.min(prefix_sum);
            max_prefix_sum = max_prefix_sum.max(prefix_sum);
        }

        return max_prefix_sum - min_prefix_sum;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, -3, 2, 3, 4],
        },
        Input {
            nums: vec![2, -5, 1, -4, 3, -2],
        },
    ];

    for input in inputs {
        let result = Solution::max_absolute_sum(input.nums);
        println!("{result:?}");
    }
}
