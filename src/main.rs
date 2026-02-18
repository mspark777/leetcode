struct Solution;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = Vec::<char>::with_capacity(chars.len());
        let n = chars.len();
        let k = k as usize;

        for i in 0..n {
            let j = (i + k) % n;
            result.push(chars[j]);
        }

        result.iter().collect()
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "dart".to_string(),
            k: 3,
        },
        Input {
            s: "aaa".to_string(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::get_encrypted_string(input.s, input.k);
        println!("{:?}", result);
    }
}
