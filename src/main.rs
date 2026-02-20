struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let sum = nums.into_iter().fold((0, 0), |acc, n| match n > 9 {
            true => (acc.0, acc.1 + n),
            _ => (acc.0 + n, acc.1),
        });

        sum.0 != sum.1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4, 10].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 5, 14].to_vec(),
        },
        Input {
            nums: [5, 5, 5, 25].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::can_alice_win(input.nums);
        println!("{:?}", result);
    }
}
