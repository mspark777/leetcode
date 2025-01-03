struct Solution {}

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0i64;
        let mut right_sum = 0i64;

        for num in nums.iter().cloned() {
            right_sum += num as i64;
        }

        let mut result = 0;
        for num in nums.iter().take(nums.len() - 1).cloned() {
            left_sum += num as i64;
            right_sum -= num as i64;

            if left_sum >= right_sum {
                result += 1;
            }
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![10, 4, -8, 7],
        },
        Input {
            nums: vec![2, 3, 1, 0],
        },
    ];

    for input in inputs {
        let result = Solution::ways_to_split_array(input.nums);
        println!("{result:?}");
    }
}
