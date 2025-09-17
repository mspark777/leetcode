struct Solution {}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        let mut result = true;
        for chunk in s.split_whitespace() {
            if let Ok(num) = chunk.parse::<i32>() {
                if num <= prev {
                    result = false;
                    break;
                } else {
                    prev = num;
                }
            }
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string(),
        },
        Input {
            s: "hello world 5 x 5".to_string(),
        },
        Input {
            s: "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::are_numbers_ascending(input.s);
        println!("{:?}", result);
    }
}
