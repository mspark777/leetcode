struct Solution;

impl Solution {
    pub fn score_balance(s: String) -> bool {
        const A: i32 = 'a' as i32;
        let mut total = 0;
        for ch in s.chars() {
            let code = ch as i32;
            let score = code + 1 - A;
            total += score;
        }

        if (total & 1) == 1 {
            return false;
        }

        let target = total / 2;
        for ch in s.chars() {
            let code = ch as i32;
            let score = code + 1 - A;
            total -= score;

            if target == total {
                return true;
            }
        }

        false
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "adcb".to_string(),
        },
        Input {
            s: "bace".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::score_balance(input.s);
        println!("{:?}", result);
    }
}
