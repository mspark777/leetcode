struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        use std::collections::HashMap;

        let mut counts = HashMap::<char, i32>::new();
        for ch in s.chars() {
            counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        let idx_map = [
            ('z', 0),
            ('w', 2),
            ('u', 4),
            ('x', 6),
            ('g', 8),
            ('o', 1),
            ('h', 3),
            ('f', 5),
            ('s', 7),
            ('i', 9),
        ];
        let mut repeats = [0; 10];
        for (l, i) in idx_map.into_iter() {
            repeats[i] = counts.get(&l).copied().unwrap_or_default();
        }

        repeats[1] -= repeats[0] + repeats[2] + repeats[4];
        repeats[3] -= repeats[8];
        repeats[5] -= repeats[4];
        repeats[7] -= repeats[6];
        repeats[9] -= repeats[5] + repeats[6] + repeats[8];

        repeats
            .into_iter()
            .enumerate()
            .map(|(i, r)| match r > 0 {
                true => i.to_string().repeat(r as usize),
                _ => String::new(),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "owoztneoer".to_string(),
        },
        Input {
            s: "fivefuro".to_string(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::original_digits(input.s);
        println!("{:?}", result);
    }
}
