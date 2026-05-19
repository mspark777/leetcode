struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let n = nums.len();
        if n < 3 {
            return false;
        }

        let mut min_array = vec![0; n];
        min_array[0] = nums[0];

        for i in 1..n {
            min_array[i] = min_array[i - 1].min(nums[i]);
        }

        let mut k = n;
        for j in (1..n).rev() {
            if nums[j] <= min_array[j] {
                continue;
            }

            while (k < n) && (nums[k] <= min_array[j]) {
                k += 1;
            }

            if (k < n) && (nums[k] < nums[j]) {
                return true;
            }

            k -= 1;
            nums[k] = nums[j];
        }

        false
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [3, 1, 4, 2].to_vec(),
        },
        Input {
            nums: [-1, 3, 2, 0].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find132pattern(input.nums);
        println!("{:?}", result);
    }
}
