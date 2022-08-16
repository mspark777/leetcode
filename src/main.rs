struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const ACODE: u8 = b'a';
        let mut memo = vec![0; 26];

        for ch in s.bytes() {
            let i = (ch - ACODE) as usize;
            memo[i] += 1;
        }

        for (i, ch) in s.bytes().enumerate() {
            let j = (ch - ACODE) as usize;
            if memo[j] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { s: "leetcode" },
        Input { s: "loveleetcode" },
        Input { s: "aabb" },
    ];

    for input in inputs {
        let result = Solution::first_uniq_char(input.s.to_string());
        println!("{:?}", result);
    }
}
