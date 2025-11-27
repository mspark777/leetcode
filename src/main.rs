struct Solution {}

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let max = nums.iter().copied().max().unwrap_or(0);

        (max * k) + ((k * (k - 1)) / 2)
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
            k: 3,
        },
        Input {
            nums: [5, 5, 5].to_vec(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::maximize_sum(input.nums, input.k);
        println!("{:?}", result);
    }
}
