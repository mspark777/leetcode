struct Solution {}

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| match s.parse::<i32>() {
                Ok(i) => i,
                _ => s.len() as i32,
            })
            .max()
            .unwrap_or(0)
    }
}

struct Input {
    strs: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            strs: ["alic3", "bob", "3", "4", "00000"]
                .map(|s| s.to_string())
                .to_vec(),
        },
        Input {
            strs: ["1", "01", "001", "0001"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::maximum_value(input.strs);
        println!("{:?}", result);
    }
}
