struct Solution {}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut result = 0;

        for n in low..=high {
            if n < 100 {
                if (n % 11) == 0 {
                    result += 1;
                }
            } else if n < 1000 {
                continue;
            } else if n >= 10000 {
                continue;
            }

            let left = (n / 1000) + ((n % 1000) / 100);
            let right = ((n % 100) / 10) + (n % 10);
            if left == right {
                result += 1;
            }
        }

        return result;
    }
}

struct Input {
    low: i32,
    high: i32,
}

fn main() {
    let inputs = vec![
        Input { low: 1, high: 100 },
        Input {
            low: 1200,
            high: 1230,
        },
    ];

    for input in inputs {
        let result = Solution::count_symmetric_integers(input.low, input.high);
        println!("{result:?}");
    }
}
