struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let mut max_len = [0; 26];
        let mut current_max = 0;
        let s = s.as_bytes();
        let n = s.len();

        for i in 0..n {
            if i > 0 {
                let curr = s[i] as i32;
                let prev = s[i - 1] as i32;
                if ((curr - prev) == 1) || ((prev - curr) == 25) {
                    current_max += 1;
                } else {
                    current_max = 1;
                }
            } else {
                current_max = 1;
            }

            let index = (s[i] - b'a') as usize;
            if current_max > max_len[index] {
                max_len[index] = current_max;
            }
        }

        max_len.iter().sum()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input { s: "a".to_string() },
        Input {
            s: "cac".to_string(),
        },
        Input {
            s: "zab".to_string(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_substring_in_wrapround_string(input.s);
        println!("{:?}", result);
    }
}
