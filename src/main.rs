struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::<bool>::with_capacity(nums.len());
        let mut n = 0;
        for num in nums.iter().cloned() {
            n = (n * 2 + num) % 5;
            result.push(n == 0);
        }
        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![0, 1, 1],
        },
        Input {
            nums: vec![1, 1, 1],
        },
    ];

    for input in inputs {
        let result = Solution::prefixes_div_by5(input.nums);
        println!("{result:?}");
    }
}
