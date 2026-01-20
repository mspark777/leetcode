struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut frequencies = [0; 26];
        let mut left = 0usize;
        let mut result = 0;

        for right in 0..s.len() {
            let idx = (s[right] as usize) - ('a' as usize);
            frequencies[idx] += 1;

            while frequencies[idx] > 2 {
                let i = (s[left] as usize) - ('a' as usize);
                frequencies[i] -= 1;
                left += 1;
            }

            let l = right + 1 - left;
            result = result.max(l as i32);
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "bcbbbcba".to_string(),
        },
        Input {
            s: "aaaa".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::maximum_length_substring(input.s);
        println!("{:?}", result);
    }
}
