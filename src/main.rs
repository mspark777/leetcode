struct Solution {}

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut less_idx = 0usize;
        let mut greater_idx = nums.len() - 1;
        let mut i = less_idx;
        let mut j = greater_idx;
        while i < nums.len() {
            if nums[i] < pivot {
                result[less_idx] = nums[i];
                less_idx += 1;
            }

            if nums[j] > pivot {
                result[greater_idx] = nums[j];
                greater_idx -= 1;
            }

            i += 1;
            if j > 0 {
                j -= 1;
            }
        }

        while less_idx <= greater_idx {
            result[less_idx] = pivot;
            less_idx += 1;
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    pivot: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![9, 12, 5, 10, 14, 3, 10],
            pivot: 10,
        },
        Input {
            nums: vec![-3, 4, 3, 2],
            pivot: 2,
        },
    ];

    for input in inputs {
        let result = Solution::pivot_array(input.nums, input.pivot);
        println!("{result:?}");
    }
}
