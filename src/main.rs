struct Solution {}
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        let mut tokens = tokens;
        let mut power = power;

        tokens.sort_unstable();

        let mut score = 0;
        let mut result = 0;
        let mut i = 0usize;
        let mut j = tokens.len() - 1;

        while (i <= j) && ((power >= tokens[i]) || (score > 0)) {
            while (i <= j) && (power >= tokens[i]) {
                power -= tokens[i];
                i += 1;
                score += 1;
            }

            result = result.max(score);

            if (i <= j) && (score > 0) {
                power += tokens[j];
                j -= 1;
                score -= 1;
            }
        }

        result
    }
}

struct Input {
    tokens: Vec<i32>,
    power: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            tokens: vec![100],
            power: 50,
        },
        Input {
            tokens: vec![100, 200],
            power: 150,
        },
        Input {
            tokens: vec![100, 200, 300, 400],
            power: 200,
        },
    ];

    for Input { tokens, power } in inputs.into_iter() {
        let result = Solution::bag_of_tokens_score(tokens, power);
        println!("{result}");
    }
}
