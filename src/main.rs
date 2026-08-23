struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut curr_max = nums[0];
        let mut possible_max = nums[0];
        let mut result = 1;

        for (i, num) in nums.into_iter().enumerate().skip(1) {
            if num < curr_max {
                result = (i + 1) as i32;
                curr_max = possible_max;
            } else {
                possible_max = possible_max.max(num);
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [5, 0, 3, 8, 6].to_vec(),
    }];

    for input in inputs {
        let result = Solution::partition_disjoint(input.nums);
        println!("{:?}", result);
    }
}
