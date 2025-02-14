struct Solution {}

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0usize;
        let mut right = s.len();
        let mut result = vec![0i32; right + 1];

        for (&ch, perm) in s.iter().zip(result.iter_mut()) {
            if ch == 'I' {
                *perm = left as i32;
                left += 1;
            } else {
                *perm = right as i32;
                right -= 1;
            }
        }

        result[s.len()] = left as i32;
        return result;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "IDID" }, Input { s: "III" }, Input { s: "DDI" }];

    for input in inputs {
        let result = Solution::di_string_match(input.s.to_string());
        println!("{result:?}");
    }
}
