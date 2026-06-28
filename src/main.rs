struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums = nums;
        let n = nums.len();

        nums.sort();
        for i in 0..((n as i32) - 2) {
            let i = i as usize;
            if nums[i] == 0 {
                continue;
            }

            let mut k = i + 2;
            for j in (i + 1)..(n - 1) {
                while (k < n) && ((nums[i] + nums[j]) > nums[k]) {
                    k += 1;
                }

                let ik = k as i32;
                let ij = j as i32;
                result += ik - ij - 1;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [4, 2, 3, 4].to_vec(),
        },
        Input { nums: [1].to_vec() },
    ];

    for input in inputs.into_iter() {
        let result = Solution::triangle_number(input.nums);
        println!("{:?}", result);
    }
}
