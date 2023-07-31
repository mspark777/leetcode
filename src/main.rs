mod utils;

use utils::Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut results = Vec::<Vec<i32>>::new();
        let mut combinations = Vec::<i32>::new();

        candidates.sort_unstable();
        Self::backtrack(&candidates, &mut combinations, target, 0, &mut results);
        return results;
    }

    fn backtrack(
        candidates: &Vec<i32>,
        combinations: &mut Vec<i32>,
        remain: i32,
        cur: usize,
        results: &mut Vec<Vec<i32>>,
    ) {
        if remain == 0 {
            results.push(combinations.clone());
            return;
        }

        for next in cur..candidates.len() {
            if (next > cur) && (candidates[next] == candidates[next - 1]) {
                continue;
            }

            let pick = candidates[next];
            let next_remain = remain - pick;
            if next_remain < 0 {
                break;
            }

            combinations.push(pick);
            Self::backtrack(candidates, combinations, next_remain, next + 1, results);
            combinations.pop();
        }
    }
}

fn main() {
    let inputs = [(vec![10, 1, 2, 7, 6, 1, 5], 8), (vec![2, 5, 2, 1, 2], 5)];

    for (candidates, target) in inputs {
        let result = Solution::combination_sum2(candidates, target);
        println!("{result:?}");
    }
}
