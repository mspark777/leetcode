struct Solution;

impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .fold(0, |acc, (i, num)| match i & 1 {
                1 => acc - num,
                _ => acc + num,
            })
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 3, 5, 7].to_vec(),
        },
        Input {
            nums: [100].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::alternating_sum(input.nums);
        println!("{:?}", result);
    }
}
