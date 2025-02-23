struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();

        let mut k = k;

        for num in nums.iter_mut() {
            if k < 1 {
                break;
            }

            if *num < 0 {
                *num = -*num;
                k -= 1;
            } else {
                break;
            }
        }

        let mut result = 0;
        let mut min_num = i32::MAX;
        for num in nums.iter().cloned() {
            result += num;
            min_num = min_num.min(num);
        }

        return result - (k & 1) * min_num * 2;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![4, 2, 3],
            k: 1,
        },
        Input {
            nums: vec![3, -1, 0, 2],
            k: 3,
        },
        Input {
            nums: vec![2, -3, -1, 5, -4],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::largest_sum_after_k_negations(input.nums, input.k);
        println!("{result:?}");
    }
}
