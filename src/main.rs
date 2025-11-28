struct Solution {}

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut distincts = std::collections::HashSet::<i32>::with_capacity(n);

        for (i, num) in nums.iter().copied().enumerate() {
            distincts.insert(num);
            result[i] = distincts.len() as i32;
        }

        distincts.clear();

        for (i, num) in nums.iter().copied().enumerate().rev() {
            result[i] -= distincts.len() as i32;
            distincts.insert(num);
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
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            nums: [3, 2, 3, 4, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::distinct_difference_array(input.nums);
        println!("{:?}", result);
    }
}
