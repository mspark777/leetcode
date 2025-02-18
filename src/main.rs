struct Solution {}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let pattern = pattern.chars().collect::<Vec<_>>();
        let mut nums = Vec::<char>::with_capacity(pattern.len());
        let mut stack = Vec::<usize>::with_capacity(pattern.len());

        for i in 0..=pattern.len() {
            stack.push(i + 1);

            if i == pattern.len() || pattern[i] == 'I' {
                while let Some(top) = stack.pop() {
                    let zero = '0' as usize;
                    nums.push((top + zero) as u8 as char);
                }
            }
        }

        return nums.iter().collect();
    }
}

struct Input {
    pattern: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            pattern: "IIIDIDDD",
        },
        Input { pattern: "DDD" },
    ];

    for input in inputs {
        let result = Solution::smallest_number(input.pattern.to_string());
        println!("{result:?}");
    }
}
