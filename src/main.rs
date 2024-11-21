struct Solution {}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect();
        let mut result = vec![0; chars.len()];

        for (i, &ch) in chars.iter().enumerate() {
            result[i] = if ch == c { 0 } else { chars.len() as i32 };
        }

        for i in 1..chars.len() {
            result[i] = result[i].min(result[i - 1] + 1);
        }

        for i in (0..(chars.len() - 1)).rev() {
            result[i] = result[i].min(result[i + 1] + 1);
        }

        return result;
    }
}

struct Input {
    s: &'static str,
    c: char,
}

fn main() {
    let inputs = vec![
        Input {
            s: "loveleetcode",
            c: 'e',
        },
        Input { s: "aaab", c: 'b' },
    ];

    for input in inputs {
        let result = Solution::shortest_to_char(input.s.to_string(), input.c);
        println!("{result:?}");
    }
}
