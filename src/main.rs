struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut counts = [0; 10];

        for (sec, gue) in secret.chars().zip(guess.chars()) {
            if sec == gue {
                bulls += 1;
                continue;
            }

            let s = Self::to_idx(sec);
            if counts[s] < 0 {
                cows += 1;
            }

            let g = Self::to_idx(gue);
            if counts[g] > 0 {
                cows += 1;
            }

            counts[s] += 1;
            counts[g] -= 1;
        }

        format!("{}A{}B", bulls, cows)
    }

    fn to_idx(ch: char) -> usize {
        let code = (ch as u8) - b'0';
        code as usize
    }
}

struct Input {
    secret: String,
    guess: String,
}

fn main() {
    let inputs = [
        Input {
            secret: "1807".to_string(),
            guess: "7810".to_string(),
        },
        Input {
            secret: "1123".to_string(),
            guess: "0111".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::get_hint(input.secret, input.guess);
        println!("{:?}", result);
    }
}
