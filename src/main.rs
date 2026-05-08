struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        use std::collections::HashMap;

        let mut depth_map = HashMap::<usize, usize>::new();
        let mut max_len = 0;

        for line in input.split('\n') {
            let level = line
                .as_bytes()
                .iter()
                .enumerate()
                .rev()
                .find(|b| (*b.1) == b'\t')
                .map(|b| b.0 + 1)
                .unwrap_or_default();
            let name = &line[level..];
            let level_len = depth_map.get(&level).copied().unwrap_or_default();

            if name.chars().find(|&c| c == '.').is_some() {
                max_len = max_len.max(level_len + name.len());
            } else {
                depth_map.insert(level + 1, level_len + name.len() + 1);
            }
        }

        max_len as i32
    }
}

struct Input {
    input: String,
}

fn main() {
    let inputs = [
        Input {
            input: "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()
        },
        Input {
            input:"dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::length_longest_path(input.input);
        println!("{:?}", result);
    }
}
