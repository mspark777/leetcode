struct Solution;

impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut result = 0;
        for (i, num) in nums.iter().copied().enumerate() {
            let left = match i < k {
                true => true,
                _ => num > nums[i - k],
            };

            let right = match (i + k) < n {
                true => num > nums[i + k],
                _ => true,
            };

            if left && right {
                result += num;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 3, 2, 1, 5, 4].to_vec(),
            k: 2,
        },
        Input {
            nums: [2, 1].to_vec(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::sum_of_good_numbers(input.nums, input.k);
        println!("{:?}", result);
    }
}
