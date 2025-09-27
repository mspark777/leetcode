struct Solution {}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sort_by_num = nums.iter().cloned().enumerate().collect::<Vec<_>>();
        sort_by_num.sort_by_key(|v| -v.1);

        let mut sort_by_idx = sort_by_num.iter().take(k as usize).collect::<Vec<_>>();
        sort_by_idx.sort_by_key(|v| v.0);
        sort_by_idx.iter().map(|v| v.1).collect()
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 1, 3, 3].to_vec(),
            k: 2,
        },
        Input {
            nums: [-1, -2, 3, 4].to_vec(),
            k: 3,
        },
        Input {
            nums: [3, 4, 3, 3].to_vec(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::max_subsequence(input.nums, input.k);
        println!("{:?}", result);
    }
}
