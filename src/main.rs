struct Solution;

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut n = 0usize;
        let mut i = 0usize;
        let mut left = s.chars().next().unwrap();

        for (j, right) in s.chars().enumerate() {
            n += 1;
            if left == right {
                continue;
            } else if (j - i) == k {
                return true;
            }

            left = right;
            i = j;
        }

        (n - i) == k
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "aaabaaa".to_string(),
            k: 3,
        },
        Input {
            s: "abc".to_string(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::has_special_substring(input.s, input.k);
        println!("{:?}", result);
    }
}
