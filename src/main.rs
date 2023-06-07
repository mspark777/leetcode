mod utils;

use utils::Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let d = (a | b) ^ c;
        let e = a & b & d;
        let result = d.count_ones() + e.count_ones();
        return result as i32;
    }
}

fn main() {
    let inputs = [(2, 6, 5), (4, 2, 7), (1, 2, 3), (7, 3, 9)];

    for (a, b, c) in inputs {
        let result = Solution::min_flips(a, b, c);
        println!("{result}");
    }
}
