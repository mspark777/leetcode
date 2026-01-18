struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().copied().filter(|&n| n < k).count() as i32
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 11, 10, 1, 3].to_vec(),
            k: 10,
        },
        Input {
            nums: [1, 1, 2, 4, 9].to_vec(),
            k: 1,
        },
        Input {
            nums: [1, 1, 2, 4, 9].to_vec(),
            k: 9,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{:?}", result);
    }
}
