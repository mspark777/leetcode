struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut count = 0;
        let mut last_ch = s.chars().next().unwrap();
        for ch in s.chars() {
            if last_ch == ch {
                count += 1;
                continue;
            }

            last_ch = ch;
            if ch == '1' {
                zero_count = zero_count.max(count);
            } else {
                one_count = one_count.max(count);
            }
            count = 1;
        }

        if last_ch == '1' {
            one_count.max(count) > zero_count
        } else {
            one_count > zero_count.max(count)
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "1101".to_string(),
        },
        Input {
            s: "111000".to_string(),
        },
        Input {
            s: "110100010".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::check_zero_ones(input.s);
        println!("{:?}", result);
    }
}
