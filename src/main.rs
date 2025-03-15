struct Solution {}

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_reward = 1;
        let mut max_reward = nums.iter().cloned().max().unwrap();

        while min_reward < max_reward {
            let mid_reward = (min_reward + max_reward) / 2;
            let mut possitle_thefts = 0;

            let mut idx = 0usize;
            while idx < nums.len() {
                let money = nums[idx];
                if money <= mid_reward {
                    possitle_thefts += 1;
                    idx += 1;
                }

                idx += 1;
            }

            if possitle_thefts >= k {
                max_reward = mid_reward;
            } else {
                min_reward = mid_reward + 1;
            }
        }

        return min_reward;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 3, 5, 9],
            k: 2,
        },
        Input {
            nums: vec![2, 7, 9, 3, 1],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::min_capability(input.nums, input.k);
        println!("{result:?}");
    }
}
