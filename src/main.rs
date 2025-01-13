struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut frequencies = vec![0; 26];
        let mut result = 0;

        for ch in s.chars() {
            let code = ch as usize;
            let a = 'a' as usize;
            let idx = code - a;
            frequencies[idx] += 1;
        }

        for frequency in frequencies.iter().cloned() {
            if frequency == 0 {
                continue;
            } else if frequency & 1 == 1 {
                result += 1;
            } else {
                result += 2;
            }
        }

        return result;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "abaacbcbb" }, Input { s: "aa" }];

    for input in inputs {
        let result = Solution::minimum_length(input.s.to_string());
        println!("{result:?}");
    }
}
