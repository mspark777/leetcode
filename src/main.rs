struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashSet::<i32>::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate().rev() {
            if seen.contains(num) {
                let i = i as i32;
                return i / 3 + 1;
            } else {
                seen.insert(*num);
            }
        }

        return 0;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 3, 4, 2, 3, 3, 5, 7],
        },
        Input {
            nums: vec![4, 5, 6, 4, 4],
        },
        Input {
            nums: vec![6, 7, 8, 9],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_operations(input.nums);
        println!("{result:?}");
    }
}
