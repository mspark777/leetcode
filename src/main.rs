struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut count = 1;
        let mut pre_count = 0;
        let mut result = 0;

        for i in 1..n {
            if nums[i] > nums[i - 1] {
                count += 1;
            } else {
                pre_count = count;
                count = 1;
            }
            result = result.max(pre_count.min(count)).max(count / 2);
        }

        result >= k
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 5, 7, 8, 9, 2, 3, 4, 3, 1].to_vec(),
            k: 3,
        },
        Input {
            nums: [1, 2, 3, 4, 4, 4, 4, 5, 6, 7].to_vec(),
            k: 5,
        },
    ];

    for input in inputs {
        let result = Solution::has_increasing_subarrays(input.nums, input.k);
        println!("{:?}", result);
    }
}
