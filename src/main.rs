use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashMap::<String, Vec<i32>>::new();
        let mut sequence = Vec::<i32>::new();
        Self::backtrack(&nums, 0, &mut sequence, &mut result);

        return result.into_values().collect();
    }

    fn backtrack(
        nums: &Vec<i32>,
        index: usize,
        sequence: &mut Vec<i32>,
        result: &mut HashMap<String, Vec<i32>>,
    ) {
        if index == nums.len() {
            if sequence.len() >= 2 {
                let key: String = sequence.iter().map(|i| format!("{i},")).collect();
                result.entry(key).or_insert(sequence.clone());
            }
        } else {
            let num = nums[index];
            let lastseq = *sequence.last().unwrap_or(&num);

            if lastseq <= num {
                sequence.push(num);
                Self::backtrack(nums, index + 1, sequence, result);
                sequence.pop();
            }
            Self::backtrack(nums, index + 1, sequence, result);
        }
    }
}

fn main() {
    let inputs = [vec![4, 6, 7, 7], vec![4, 4, 3, 2, 1]];

    for nums in inputs {
        let result = Solution::find_subsequences(nums);
        println!("{result:?}");
    }
}
