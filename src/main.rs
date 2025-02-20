struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = Vec::<char>::with_capacity(nums.len());

        for i in 0..nums.len() {
            let ch = nums[i].chars().nth(i).unwrap();
            if ch == '1' {
                result.push('0');
            } else {
                result.push('1');
            }
        }

        return result.iter().collect();
    }
}

struct Input {
    nums: Vec<&'static str>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec!["01", "10"],
        },
        Input {
            nums: vec!["00", "01"],
        },
        Input {
            nums: vec!["111", "011", "001"],
        },
    ];

    for input in inputs {
        let result = Solution::find_different_binary_string(
            input.nums.iter().map(|s| s.to_string()).collect(),
        );
        println!("{result:?}");
    }
}
