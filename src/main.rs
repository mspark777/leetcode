struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = std::collections::HashMap::<i32, usize>::with_capacity(nums.len());
        let mut result = nums.clone();
        result.sort_unstable();

        for (i, n) in result.iter().cloned().enumerate() {
            counts.entry(n).or_insert(i);
        }

        for (i, n) in nums.iter().cloned().enumerate() {
            let &idx = counts.get(&n).unwrap();
            result[i] = idx as i32;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [8, 1, 2, 2, 3].to_vec(),
        },
        Input {
            nums: [6, 5, 4, 8].to_vec(),
        },
        Input {
            nums: [7, 7, 7, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::smaller_numbers_than_current(input.nums);
        println!("{:?}", result);
    }
}
