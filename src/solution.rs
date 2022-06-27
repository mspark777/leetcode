pub struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap_or('0').to_digit(10).unwrap_or(0) as i32
    }
}
