struct Solution {}

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut counts = vec![0; 26];
        let mut s_len = 0usize;
        for c in s.chars() {
            let c = c as u8;
            let a = b'a';
            let i = (c - a) as usize;
            counts[i] += 1;
            s_len += 1;
        }

        let mut stack = Vec::<u8>::with_capacity(s_len);
        let mut result = Vec::<char>::with_capacity(s_len);
        let mut min_char = b'a';
        for c in s.chars() {
            let c = c as u8;
            stack.push(c);
            counts[(c - b'a') as usize] -= 1;
            while min_char != b'z' && counts[(min_char - b'a') as usize] == 0 {
                min_char += 1;
            }

            while let Some(&ch) = stack.last() {
                if ch <= min_char {
                    result.push(ch as char);
                    stack.pop();
                } else {
                    break;
                }
            }
        }

        return result.iter().collect();
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "zza" }, Input { s: "bac" }, Input { s: "bdda" }];

    for input in inputs {
        let result = Solution::robot_with_string(input.s.to_string());
        println!("{:?}", result);
    }
}
