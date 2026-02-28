struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let l = l as usize;
        let r = r as usize;
        let n = nums.len();
        let mut min_sum = i32::MAX;
        let mut found = false;

        for k in l..=r {
            let mut sum = nums.iter().copied().take(k).sum::<i32>();
            if sum > 0 {
                min_sum = min_sum.min(sum);
                found = true;
            }

            for i in k..n {
                sum += nums[i];
                sum -= nums[i - k];

                if sum > 0 {
                    min_sum = min_sum.min(sum);
                    found = true;
                }
            }
        }

        match found {
            true => min_sum,
            _ => -1,
        }
    }
}

struct Input {
    nums: Vec<i32>,
    l: i32,
    r: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, -2, 1, 4].to_vec(),
            l: 2,
            r: 3,
        },
        Input {
            nums: [-2, 2, -3, 1].to_vec(),
            l: 2,
            r: 3,
        },
        Input {
            nums: [1, 2, 3, 4].to_vec(),
            l: 2,
            r: 4,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_sum_subarray(input.nums, input.l, input.r);
        println!("{:?}", result);
    }
}
