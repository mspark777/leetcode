struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashSet;

        let s = s.chars().collect::<Vec<char>>();
        if s.len() < 10 {
            return Vec::new();
        }

        let mut seen = HashSet::<String>::new();
        let mut repeated = HashSet::<String>::new();
        let end = s.len() - 9;
        for i in 0..end {
            let e = i + 10;
            let sub = s[i..e].iter().collect::<String>();
            if seen.contains(&sub) {
                repeated.insert(sub);
            } else {
                seen.insert(sub);
            }
        }

        repeated.into_iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string(),
        },
        Input {
            s: "AAAAAAAAAAAAA".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_repeated_dna_sequences(input.s);
        println!("{:?}", result);
    }
}
