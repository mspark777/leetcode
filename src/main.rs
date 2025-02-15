struct Solution {}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 1..=3 {
            for j in 0..(nums.len() - i) {
                if nums[j] == nums[j + i] {
                    return nums[j];
                }
            }
        }

        return -1;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 3, 3],
        },
        Input {
            nums: vec![2, 1, 2, 5, 3, 2],
        },
        Input {
            nums: vec![5, 1, 5, 2, 5, 3, 5, 4],
        },
    ];

    for input in inputs {
        let result = Solution::repeated_n_times(input.nums);
        println!("{result:?}");
    }
}
