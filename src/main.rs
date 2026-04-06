struct Solution;

impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut left = 0usize;
        let mut right = s.len() - 1;

        while left < right {
            let l = s[left].is_ascii_lowercase();
            let r = s[right].is_ascii_lowercase();

            if l && r {
                let tl = s[left];
                let rl = s[right];
                s[left] = rl;
                s[right] = tl;
                left += 1;
                right -= 1;
            } else if l {
                right -= 1;
            } else {
                left += 1;
            }
        }

        left = 0;
        right = s.len() - 1;
        while left < right {
            let l = !s[left].is_ascii_lowercase();
            let r = !s[right].is_ascii_lowercase();

            if l && r {
                let tl = s[left];
                let rl = s[right];
                s[left] = rl;
                s[right] = tl;
                left += 1;
                right -= 1;
            } else if l {
                right -= 1;
            } else {
                left += 1;
            }
        }

        s.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: ")ebc#da@f(".to_string(),
        },
        Input { s: "z".to_string() },
        Input {
            s: "!@#$%^&*()".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::reverse_by_type(input.s);
        println!("{:?}", result);
    }
}
