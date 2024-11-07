struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..24 {
            let mut count = 0;
            for &num in candidates.iter() {
                let mask = num & (1 << i);
                if mask != 0 {
                    count += 1;
                }
            }

            result = result.max(count);
        }

        return result;
    }
}

struct Input {
    candidates: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            candidates: vec![16, 17, 71, 62, 12, 24, 14],
        },
        Input {
            candidates: vec![8, 8],
        },
    ];

    for input in inputs {
        let result = Solution::largest_combination(input.candidates);
        println!("{result}");
    }
}
