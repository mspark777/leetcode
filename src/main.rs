struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut result = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if (((i * j) % k) == 0) && (nums[i] == nums[j]) {
                    result += 1;
                }
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
            nums: [3, 1, 2, 2, 2, 1, 3].to_vec(),
            k: 2,
        },
        Input {
            nums: [1, 2, 3, 4].to_vec(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::count_pairs(input.nums, input.k);
        println!("{:?}", result);
    }
}
