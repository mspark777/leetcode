struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        match nums.len() {
            0 => String::new(),
            1 => nums[0].to_string(),
            2 => format!("{}/{}", nums[0], nums[1]),
            _ => format!(
                "{}/({})",
                nums[0],
                nums[1..]
                    .iter()
                    .copied()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("/")
            ),
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [1000, 100, 10, 2].to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::optimal_division(input.nums);
        println!("{:?}", result);
    }
}
