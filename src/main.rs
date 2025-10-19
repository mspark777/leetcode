struct Solution {}

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut prev = 0;
        let mut result = 0.0f64;

        for bracket in brackets.iter() {
            let upper = bracket[0].min(income);
            let percent = (bracket[1] as f64) / 100.0;
            let range = upper - prev;
            prev = upper;
            result += range as f64 * percent;

            if upper >= income {
                break;
            }
        }

        result
    }
}

struct Input {
    brackets: Vec<Vec<i32>>,
    income: i32,
}

fn main() {
    let inputs = [
        Input {
            brackets: [[3, 50], [7, 10], [12, 25]].map(|v| v.to_vec()).to_vec(),
            income: 10,
        },
        Input {
            brackets: [[1, 0], [4, 25], [5, 50]].map(|v| v.to_vec()).to_vec(),
            income: 2,
        },
        Input {
            brackets: [[2, 50]].map(|v| v.to_vec()).to_vec(),
            income: 0,
        },
    ];

    for input in inputs {
        let result = Solution::calculate_tax(input.brackets, input.income);
        println!("{:?}", result);
    }
}
