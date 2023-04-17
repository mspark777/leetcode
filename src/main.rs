mod utils;

use utils::Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candy = candies.iter().max().unwrap() - extra_candies;
        return candies.iter().map(|&c| c >= max_candy).collect();
    }
}

fn main() {
    let inputs = [
        (vec![2, 3, 5, 1, 3], 3),
        (vec![4, 2, 1, 1, 2], 1),
        (vec![12, 1, 12], 10),
    ];

    for (candies, extra_candies) in inputs {
        let result = Solution::kids_with_candies(candies, extra_candies);
        println!("{result:?}");
    }
}
