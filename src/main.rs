struct Solution {}

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut left = 0usize;
        let mut right = chars.len() - 1;

        while left < right {
            let lc = chars[left];
            let rc = chars[right];

            if lc < rc {
                chars[right] = lc;
            } else if lc > rc {
                chars[left] = rc;
            }

            left += 1;
            right -= 1;
        }

        chars.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "egcfe".to_string(),
        },
        Input {
            s: "abcd".to_string(),
        },
        Input {
            s: "seven".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::make_smallest_palindrome(input.s);
        println!("{:?}", result);
    }
}
