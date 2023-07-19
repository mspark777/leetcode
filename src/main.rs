mod utils;

use utils::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut temp = Vec::<i32>::new();
        Self::solve(0, &candidates, &mut temp, target, &mut result);

        return result;
    }

    fn solve(
        i: usize,
        candidates: &Vec<i32>,
        temp: &mut Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(temp.clone());
            return;
        }

        if target < 0 {
            return;
        }

        if i >= candidates.len() {
            return;
        }

        Self::solve(i + 1, candidates, temp, target, result);

        let candidate = candidates[i];
        temp.push(candidate);
        Self::solve(i, candidates, temp, target - candidate, result);
        temp.pop();
    }
}

fn main() {
    let inputs = [(vec![2, 3, 6, 7], 7), (vec![2, 3, 5], 8), (vec![2], 1)];

    for (candidates, target) in inputs {
        let result = Solution::combination_sum(candidates, target);
        println!("{result:?}");
    }
}
