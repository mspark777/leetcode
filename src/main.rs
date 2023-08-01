mod utils;

use utils::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();
        let mut combinations = Vec::<i32>::new();
        Self::backtrack(1, &mut combinations, &mut results, n as usize, k as usize);

        return results;
    }

    fn backtrack(
        start: usize,
        combinations: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
        n: usize,
        k: usize,
    ) {
        if combinations.len() == k {
            results.push(combinations.clone());
            return;
        }

        for i in start..=n {
            combinations.push(i as i32);
            Self::backtrack(i + 1, combinations, results, n, k);
            combinations.pop();
        }
    }
}

fn main() {
    let inputs = [(4, 2), (1, 1)];

    for (n, k) in inputs {
        let result = Solution::combine(n, k);
        println!("{result:?}");
    }
}
