struct Solution;

impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut freqs = [0; 26];
        for c in s.bytes() {
            freqs[(c - b'a') as usize] += 1;
        }
        let mut fprim = vec![0; 101];
        for f in freqs.iter() {
            fprim[*f as usize] += 1;
        }
        let max_prim = fprim[1..].iter().max().unwrap();
        let freq_grp = fprim.iter().rposition(|x| x == max_prim).unwrap() as i32;
        let mut res = String::new();
        for (idx, f) in freqs.iter().enumerate() {
            if *f == freq_grp {
                res.push((idx as u8 + b'a') as char)
            }
        }

        res
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "aaabbbccdddde".to_string(),
        },
        Input {
            s: "abcd".to_string(),
        },
        Input {
            s: "pfpfgi".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::majority_frequency_group(input.s);
        println!("{:?}", result);
    }
}
