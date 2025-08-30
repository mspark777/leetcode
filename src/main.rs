struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut counts = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
        for num in nums.iter().cloned() {
            counts.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        counts
            .iter()
            .filter(|&(_, &count)| count == 1)
            .map(|(&num, _)| num)
            .sum()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 2].to_vec(),
        },
        Input {
            nums: [1, 1, 1, 1, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sum_of_unique(input.nums);
        println!("{:?}", result);
    }
}
