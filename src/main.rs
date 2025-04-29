struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_num = nums.iter().cloned().max().unwrap();
        let mut result = 0usize;
        let mut start = 0usize;
        let mut k = k;

        for end in 0..nums.len() {
            if nums[end] == max_num {
                k -= 1;
            }

            while k == 0 {
                if nums[start] == max_num {
                    k = 1;
                }

                start += 1;
            }

            result += start;
        }

        return result as i64;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 3, 2, 3, 3],
            k: 2,
        },
        Input {
            nums: vec![1, 4, 2, 1],
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::count_subarrays(input.nums, input.k);
        println!("{result:?}");
    }
}
