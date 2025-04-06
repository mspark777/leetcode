struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort_unstable();

        let n = nums.len();
        let mut prev = vec![Vec::<i32>::new(); n + 1];

        for i in 1..=n {
            let mut curr = vec![Vec::<i32>::new(); n + 1];
            for j in 1..=n {
                let mut x = Vec::<i32>::new();
                if j == 1 {
                    x = prev[i].clone();
                    x.push(nums[i - 1]);
                } else if (nums[j - 1] % nums[i - 1]) == 0 {
                    x = prev[i].clone();
                    x.push(nums[i - 1]);
                } else if (nums[i - 1] % nums[j - 1]) == 0 {
                    x = prev[i].clone();
                    x.push(nums[i - 1]);
                }

                let y = &prev[j];
                if y.len() > x.len() {
                    curr[j] = y.clone();
                } else {
                    curr[j] = x.clone();
                }
            }

            prev = curr;
        }

        return prev[1].clone();
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 3],
        },
        Input {
            nums: vec![1, 2, 4, 8],
        },
    ];

    for input in inputs {
        let result = Solution::largest_divisible_subset(input.nums);
        println!("{result:?}");
    }
}
