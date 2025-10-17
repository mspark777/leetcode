struct Solution {}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut s_counts = vec![0; 26];
        for c in s.chars() {
            let code = c as usize;
            let a = 'a' as usize;
            s_counts[code - a] += 1;
        }

        let mut target_counts = vec![0; 26];
        for c in target.chars() {
            let code = c as usize;
            let a = 'a' as usize;
            target_counts[code - a] += 1;
        }

        let mut result = i32::MAX;
        for (s_cnt, t_cnt) in s_counts.iter().cloned().zip(target_counts.iter().cloned()) {
            if t_cnt > 0 {
                result = result.min(s_cnt / t_cnt);
            }

            if result == 0 {
                break;
            }
        }

        result
    }
}

struct Input {
    s: String,
    target: String,
}

fn main() {
    let inputs = [
        Input {
            s: "ilovecodingonleetcode".to_string(),
            target: "code".to_string(),
        },
        Input {
            s: "abcba".to_string(),
            target: "abc".to_string(),
        },
        Input {
            s: "abbaccaddaeea".to_string(),
            target: "aaaaa".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::rearrange_characters(input.s, input.target);
        println!("{:?}", result);
    }
}
