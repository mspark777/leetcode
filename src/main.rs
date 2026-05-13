struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let codes = s
            .as_bytes()
            .iter()
            .copied()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<usize>>();
        let n = codes.len();
        let mut counts = [0; 26];
        let mut result = 0;
        let mut i = 0usize;
        let mut max_count = 0i32;

        for j in 0..n {
            let jcode = codes[j];
            counts[jcode] += 1;
            max_count = max_count.max(counts[jcode]);

            while ((j + 1 - i) as i32) - max_count > k {
                counts[codes[i]] -= 1;
                i += 1;
            }

            if ((j + 1 - i) as i32) > result {
                result = (j + 1 - i) as i32;
            }
        }

        result
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "ABAB".to_string(),
            k: 2,
        },
        Input {
            s: "AABABBA".to_string(),
            k: 1,
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::character_replacement(input.s, input.k);
        println!("{:?}", result);
    }
}
