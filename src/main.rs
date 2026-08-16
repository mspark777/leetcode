struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        Self::fun(&mut result, &num, 0);
        result
    }

    fn fun(result: &mut Vec<i32>, num: &str, ind: usize) -> Option<bool> {
        if ind == num.len() {
            return Some(result.len() > 2);
        }

        let mut val = 0i64;
        for i in ind..num.len() {
            val = val * 10 + (num.chars().nth(i)? as u8 - b'0') as i64;
            if val > i32::MAX as i64 {
                return Some(false);
            }
            let y = match result.len() < 2 {
                true => 0,
                _ => result[result.len() - 2] + result[result.len() - 1],
            };

            if result.len() < 2 || y == val as i32 {
                result.push(val as i32);
                if Self::fun(result, num, i + 1)? {
                    return Some(true);
                }
                result.pop();
            }
            if i == ind && num.chars().nth(i)? == '0' {
                return Some(false);
            }
        }
        Some(false)
    }
}

struct Input {
    num: String,
}

fn main() {
    let inputs = [Input {
        num: "1101111".to_string(),
    }];

    for input in inputs {
        let result = Solution::split_into_fibonacci(input.num);
        println!("{:?}", result);
    }
}
