struct Solution {}

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut odd_count = 0u32;
        let mut s_len = 0;

        for ch in s.chars() {
            let code = ch as u32;
            let a = 'a' as u32;
            odd_count ^= 1 << (code - a);
            s_len += 1;
        }

        if s_len < k {
            return false;
        } else if s_len == k {
            return true;
        }

        return odd_count.count_ones() <= (k as u32);
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            s: "annabelle",
            k: 2,
        },
        Input {
            s: "leetcode",
            k: 3,
        },
        Input { s: "true", k: 4 },
    ];

    for input in inputs {
        let result = Solution::can_construct(input.s.to_string(), input.k);
        println!("{result:?}");
    }
}
