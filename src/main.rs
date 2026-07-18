struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        (0..n)
            .map(|i| Self::count(s, i, i, n) + Self::count(s, i, i + 1, n))
            .sum()
    }

    fn count(s: &[u8], i: usize, j: usize, n: usize) -> i32 {
        (0..=i)
            .rev()
            .zip(j..n)
            .try_fold(0, |a, (i, j)| match s[i] == s[j] {
                true => Ok(a + 1),
                _ => Err(a),
            })
            .unwrap_or_else(|a| a)
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "abc".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::count_substrings(input.s);
        println!("{:?}", result);
    }
}
