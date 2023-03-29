mod utils;

use utils::Solution;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort_unstable_by_key(|k| -k);

        let mut result = 0;
        let mut suffix = 0;

        for &s in satisfaction.iter() {
            suffix += s;
            if suffix <= 0 {
                break;
            }

            result += suffix;
        }

        return result;
    }
}

fn main() {
    let inputs = [vec![-1, -8, 0, 5, -9], vec![4, 3, 2], vec![-1, -4, -5]];

    for satisfaction in inputs {
        let result = Solution::max_satisfaction(satisfaction);
        println!("{result}");
    }
}
