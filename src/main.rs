struct Solution {}

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut len = 0;
        let mut count = 0;
        for c in s.chars() {
            if letter == c {
                count += 100;
            }
            len += 1;
        }

        match count {
            0 => 0,
            _ => count / len,
        }
    }
}

struct Input {
    s: String,
    letter: char,
}

fn main() {
    let inputs = [
        Input {
            s: "foobar".to_string(),
            letter: 'o',
        },
        Input {
            s: "jjjj".to_string(),
            letter: 'k',
        },
    ];

    for input in inputs {
        let result = Solution::percentage_letter(input.s, input.letter);
        println!("{:?}", result);
    }
}
