struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, n| {
            let m = n % 3;
            acc + m.min(3 - m)
        })
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [3, 6, 9].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_operations(input.nums);
        println!("{:?}", result);
    }
}
