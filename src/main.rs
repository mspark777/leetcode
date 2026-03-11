struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 9, 7].to_vec(),
            k: 5,
        },
        Input {
            nums: [4, 1, 3].to_vec(),
            k: 4,
        },
        Input {
            nums: [3, 2].to_vec(),
            k: 6,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{:?}", result);
    }
}
