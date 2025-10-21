struct Solution {}

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut sum = 0;
        for a in amount.iter().cloned() {
            max_count = max_count.max(a);
            sum += a;
        }

        max_count.max((sum + 1) / 2)
    }
}

struct Input {
    amount: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            amount: [1, 4, 2].to_vec(),
        },
        Input {
            amount: [5, 4, 4].to_vec(),
        },
        Input {
            amount: [5, 0, 0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::fill_cups(input.amount);
        println!("{:?}", result);
    }
}
