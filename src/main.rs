struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        use std::collections::HashMap;
        let mut index_map = HashMap::<char, i32>::new();
        for (i, ch) in t.chars().enumerate() {
            index_map.insert(ch, i as i32);
        }

        let mut result = 0;
        for (i, ch) in s.chars().enumerate() {
            let i = i as i32;
            let &idx = index_map.get(&ch).unwrap();

            result += (i - idx).abs();
        }

        result
    }
}

struct Input {
    s: String,
    t: String,
}

fn main() {
    let inputs = [
        Input {
            s: "abc".to_string(),
            t: "bac".to_string(),
        },
        Input {
            s: "abcde".to_string(),
            t: "edbac".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_permutation_difference(input.s, input.t);
        println!("{:?}", result);
    }
}
