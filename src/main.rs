struct Solution {}

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let chars1 = str1.chars().collect::<Vec<char>>();
        let chars2 = str2.chars().collect::<Vec<char>>();
        let mut str2_idx = 0usize;
        let str1_len = chars1.len();
        let str2_len = chars2.len();

        for str1_idx in 0..str1_len {
            if str2_idx >= str2_len {
                break;
            }

            let ch1 = chars1[str1_idx] as u8;
            let ch2 = chars2[str2_idx] as u8;
            if ch1 == ch2 {
                str2_idx += 1;
            } else if (ch1 + 1) == ch2 {
                str2_idx += 1;
            } else if (ch1 - 25) == ch2 {
                str2_idx += 1;
            }
        }

        return str2_idx == str2_len;
    }
}

struct Input {
    str1: &'static str,
    str2: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            str1: "abc",
            str2: "ad",
        },
        Input {
            str1: "zc",
            str2: "ad",
        },
        Input {
            str1: "ab",
            str2: "d",
        },
    ];

    for input in inputs {
        let result = Solution::can_make_subsequence(input.str1.to_string(), input.str2.to_string());
        println!("{result}");
    }
}
