mod utils;

use utils::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut current = result;

        for &altitude in gain.iter() {
            current += altitude;
            result = result.max(current);
        }

        return result;
    }
}

fn main() {
    let inputs = [vec![-5, 1, 5, 0, -7], vec![-4, -3, -2, -1, 4, 3, 2]];

    for gain in inputs {
        let result = Solution::largest_altitude(gain);
        println!("{result}");
    }
}
