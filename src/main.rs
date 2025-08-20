struct Solution {}

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let nums = n.to_string().chars().collect::<Vec<char>>();
        let mut result = Vec::<char>::with_capacity(nums.len() + ((nums.len() - 1) / 3));
        let mut point = 0usize;

        for num in nums.iter().rev().cloned() {
            if point == 3 {
                result.push('.');
                result.push(num);
                point = 1;
            } else {
                result.push(num);
                point += 1;
            }
        }

        result.iter().rev().collect()
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 987 }, Input { n: 1234 }];

    for input in inputs {
        let result = Solution::thousand_separator(input.n);
        println!("{:?}", result);
    }
}
