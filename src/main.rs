mod utils;

use utils::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();
        let mut permutations = Vec::<i32>::new();
        Self::backtrack(&mut permutations, &nums, &mut results);

        return results;
    }

    fn backtrack(permutations: &mut Vec<i32>, nums: &Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if permutations.len() == nums.len() {
            results.push(permutations.clone());
            return;
        }

        for num in nums.iter() {
            if permutations.contains(num) {
                continue;
            }

            permutations.push(*num);
            Self::backtrack(permutations, nums, results);
            permutations.pop();
        }
    }
}

fn main() {
    let inputs = [vec![1, 2, 3], vec![0, 1], vec![1]];

    for nums in inputs {
        let result = Solution::permute(nums);
        println!("{result:?}");
    }
}
