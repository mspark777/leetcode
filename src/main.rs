struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let mut last_pos = [-1, -1, -1];
        let mut result = 0;

        for pos in 0..len {
            let code = Self::code(s[pos]);
            last_pos[code] = pos as i32;

            result += 1 + last_pos.iter().min().unwrap();
        }

        return result;
    }

    fn code(ch: char) -> usize {
        return ((ch as u8) - b'a') as usize;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "abcabc" },
        Input { s: "aaacb" },
        Input { s: "abc" },
    ];

    for input in inputs {
        let result = Solution::number_of_substrings(input.s.to_string());
        println!("{result:?}");
    }
}
