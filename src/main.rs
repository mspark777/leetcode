struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let last = ('z' as usize) + 1;
        let mut result = 0usize;
        for (i, ch) in s.chars().enumerate() {
            let idx = last - (ch as usize);
            result += idx * (i + 1);
        }

        result as i32
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "abc".to_string(),
        },
        Input {
            s: "zaza".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::reverse_degree(input.s);
        println!("{:?}", result);
    }
}
