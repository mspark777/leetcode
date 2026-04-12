struct Solution;

impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left <= right {
            if s[left] == s[right] {
                return left as i32;
            }

            left += 1;
            right -= 1;
        }

        -1
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "abcacbd".to_string(),
        },
        Input {
            s: "abc".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::first_matching_index(input.s);
        println!("{:?}", result);
    }
}
