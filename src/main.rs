struct Solution {}

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }

        let max = nums.iter().copied().take(3).max().unwrap();
        let min = nums.iter().copied().take(3).min().unwrap();

        nums.iter().copied().take(3).sum::<i32>() - (max + min)
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 2, 1, 4].to_vec(),
        },
        Input {
            nums: [1, 2].to_vec(),
        },
        Input {
            nums: [2, 1, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_non_min_or_max(input.nums);
        println!("{:?}", result);
    }
}
