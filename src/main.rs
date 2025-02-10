struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut result_length = 0usize;

        for i in 0..s.len() {
            let ch = s[i];
            if ch.is_digit(10) {
                if result_length > 1 {
                    result_length -= 1;
                } else {
                    result_length = 0;
                }
            } else {
                s[result_length] = ch;
                result_length += 1;
            }
        }

        return s.iter().take(result_length).collect();
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "abc" }, Input { s: "cb34" }];

    for input in inputs {
        let result = Solution::clear_digits(input.s.to_string());
        println!("{result:?}");
    }
}
