struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let set = nums
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();

        let mut result = original;
        while set.contains(&result) {
            result *= 2;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    original: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [5, 3, 6, 1, 12].to_vec(),
            original: 3,
        },
        Input {
            nums: [2, 7, 9].to_vec(),
            original: 4,
        },
    ];

    for input in inputs {
        let result = Solution::find_final_value(input.nums, input.original);
        println!("{:?}", result);
    }
}
