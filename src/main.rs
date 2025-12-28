struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut min_idx = 0usize;
        let mut max_idx = 0usize;
        let n = nums.len();
        let index_difference = index_difference as usize;

        for i in index_difference..n {
            let j = i - index_difference;
            if nums[j] < nums[min_idx] {
                min_idx = j
            }

            if nums[j] > nums[max_idx] {
                max_idx = j
            }

            if (nums[i] - nums[min_idx]) >= value_difference {
                return vec![min_idx as i32, i as i32];
            }

            if (nums[max_idx] - nums[i]) >= value_difference {
                return vec![max_idx as i32, i as i32];
            }
        }

        vec![-1, -1]
    }
}

struct Input {
    nums: Vec<i32>,
    index_difference: i32,
    value_difference: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [5, 1, 4, 1].to_vec(),
            index_difference: 2,
            value_difference: 4,
        },
        Input {
            nums: [2, 1].to_vec(),
            index_difference: 0,
            value_difference: 0,
        },
        Input {
            nums: [1, 2, 3].to_vec(),
            index_difference: 2,
            value_difference: 4,
        },
    ];

    for input in inputs {
        let result =
            Solution::find_indices(input.nums, input.index_difference, input.value_difference);
        println!("{:?}", result);
    }
}
