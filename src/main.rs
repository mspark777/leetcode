struct Solution {}

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut sums = std::collections::HashSet::<i32>::with_capacity(nums.len() - 1);
        for (left, right) in nums.iter().cloned().zip(nums.iter().skip(1).cloned()) {
            let sum = left + right;
            if sums.contains(&sum) {
                return true;
            } else {
                sums.insert(sum);
            }
        }

        return false;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [4, 2, 4].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            nums: [0, 0, 0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_subarrays(input.nums);
        println!("{:?}", result);
    }
}
