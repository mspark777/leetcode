struct Solution;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let blacklist = fronts
            .iter()
            .copied()
            .zip(backs.iter().copied())
            .filter(|(f, b)| f == b)
            .map(|v| v.0)
            .collect::<HashSet<i32>>();

        fronts
            .into_iter()
            .chain(backs)
            .filter(|v| !blacklist.contains(v))
            .min()
            .unwrap_or_default()
    }
}

struct Input {
    fronts: Vec<i32>,
    backs: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        s: "(123)".to_string(),
    }];

    for input in inputs {
        let result = Solution::ambiguous_coordinates(input.s);
        println!("{:?}", result);
    }
}
