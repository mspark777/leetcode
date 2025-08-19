struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = vec![' '; chars.len()];
        for i in 0..indices.len() {
            result[indices[i] as usize] = chars[i];
        }

        result.iter().collect()
    }
}

struct Input {
    s: String,
    indices: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            s: "codeleet".to_string(),
            indices: [4, 5, 6, 7, 0, 2, 1, 3].to_vec(),
        },
        Input {
            s: "abc".to_string(),
            indices: [0, 1, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::restore_string(input.s, input.indices);
        println!("{:?}", result);
    }
}
