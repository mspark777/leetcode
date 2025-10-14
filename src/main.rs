struct Solution {}

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let number = number.chars().collect::<Vec<char>>();
        let mut result = String::new();

        for (i, c) in number.iter().cloned().enumerate() {
            if c != digit {
                continue;
            }

            let left = &number[0..i];
            let right = &number[i + 1..];
            let temp = left.iter().chain(right.iter()).collect::<String>();
            if result.cmp(&temp) == std::cmp::Ordering::Less {
                result = temp;
            }
        }

        result
    }
}

struct Input {
    number: String,
    digit: char,
}

fn main() {
    let inputs = [
        Input {
            number: "123".to_string(),
            digit: '3',
        },
        Input {
            number: "1231".to_string(),
            digit: '1',
        },
        Input {
            number: "551".to_string(),
            digit: '5',
        },
    ];

    for input in inputs {
        let result = Solution::remove_digit(input.number, input.digit);
        println!("{:?}", result);
    }
}
