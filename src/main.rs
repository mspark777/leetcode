use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        const MOD: usize = 1000000007;
        let alen = arr.len();

        let mut arr = arr;
        arr.sort_unstable();

        let mut dp = vec![1usize; alen];
        let mut index = HashMap::<usize, usize>::with_capacity(alen);
        for (k, v) in arr.iter().enumerate() {
            let key = *v as usize;
            index.insert(key, k);
        }

        for i in 0..alen {
            let parent = arr[i] as usize;
            for j in 0..i {
                let left = arr[j] as usize;
                if (parent % left) == 0 {
                    let right = parent / left;
                    if let Some(memo) = index.get(&right) {
                        dp[i] += (dp[j] * dp[*memo]) % MOD
                    }
                }
            }
        }

        let result = dp.iter().fold(0, |acc, cur| acc + cur);
        (result % MOD) as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { nums: vec![2, 4] },
        Input {
            nums: vec![2, 4, 5, 10],
        },
    ];

    for input in inputs {
        let nums = input.nums;
        let result = Solution::num_factored_binary_trees(nums);
        println!("{:?}", result);
    }
}
