struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut unpaired = std::collections::HashSet::<i32>::with_capacity(nums.len());

        for num in nums.iter() {
            if unpaired.contains(num) {
                unpaired.remove(num);
            } else {
                unpaired.insert(*num);
            }
        }

        return unpaired.is_empty();
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 2, 3, 2, 2, 2],
        },
        Input {
            nums: vec![1, 2, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::divide_array(input.nums);
        println!("{result:?}");
    }
}
