struct Solution {}

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut balanced = 0;
        let mut count0 = 0;
        let mut count1 = 0;

        for ch in s.chars() {
            if ch == '0' {
                if count1 > 0 {
                    count0 = 1;
                } else {
                    count0 += 1;
                }
                count1 = 0;
            } else {
                count1 += 1;
            }

            if count0 >= count1 {
                balanced = balanced.max(count1);
            }
        }

        balanced * 2
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "01000111".to_string(),
        },
        Input {
            s: "00111".to_string(),
        },
        Input {
            s: "111".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_the_longest_balanced_substring(input.s);
        println!("{:?}", result);
    }
}
