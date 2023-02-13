/*
leetcode
 */

struct Solution {}
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut low = low;

        low |= 1;
        if low > high {
            return 0;
        }

        return ((high - low) / 2) + 1;
    }
}

fn main() {
    let inputs: Vec<(i32, i32)> = vec![(3, 7), (8, 10)];

    for (low, high) in inputs {
        let result = Solution::count_odds(low, high);
        println!("{result}");
    }
}
