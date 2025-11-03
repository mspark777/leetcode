struct Solution {}

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut sum_set = std::collections::HashSet::<String>::new();
        let last = nums.len() - 1;
        for l in 0..last {
            let r = last - l;
            if l >= r {
                break;
            }

            let min = nums[l];
            let max = nums[r];
            let sum = (max + min) as f64;
            sum_set.insert((sum / 2.0).to_string());
        }

        sum_set.len() as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [4, 1, 4, 0, 3, 5].to_vec(),
        },
        Input {
            nums: [1, 100].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::distinct_averages(input.nums);
        println!("{:?}", result);
    }
}
