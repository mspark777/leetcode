struct Solution;

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let k = k as usize;
        let h = k / 2;
        let mut chars = s.chars().collect::<Vec<char>>();

        for i in 0..h {
            let left = chars[i];
            let right = chars[k - i - 1];

            chars[i] = right;
            chars[k - i - 1] = left;
        }

        chars.iter().collect()
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "abcd".to_string(),
            k: 2,
        },
        Input {
            s: "xyz".to_string(),
            k: 3,
        },
        Input {
            s: "key".to_string(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::reverse_prefix(input.s, input.k);
        println!("{:?}", result);
    }
}
