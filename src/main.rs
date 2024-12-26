struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut result_map = std::collections::HashMap::<i32, i32>::new();
        result_map.insert(0, 1);

        for num in nums.iter().cloned() {
            let mut dp = std::collections::HashMap::<i32, i32>::new();

            for (&t, &sum) in result_map.iter() {
                *dp.entry(t + num).or_insert(0) += sum;
                *dp.entry(t - num).or_insert(0) += sum;
            }

            result_map = dp;
        }

        return *result_map.get(&target).unwrap_or(&0);
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 1, 1, 1, 1],
            target: 3,
        },
        Input {
            nums: vec![1],
            target: 1,
        },
    ];

    for input in inputs {
        let result = Solution::find_target_sum_ways(input.nums, input.target);
        println!("{result:?}");
    }
}
