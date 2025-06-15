struct Solution {}

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut min_num = num.to_string();
        let mut max_num = min_num.clone();
        for digit in max_num.chars() {
            if digit != '9' {
                max_num = Self::replace(max_num.as_str(), digit, '9');
                break;
            }
        }

        for (i, digit) in min_num.chars().enumerate() {
            if i == 0 {
                if digit != '1' {
                    min_num = Self::replace(min_num.as_str(), digit, '1');
                    break;
                }
            } else {
                if digit != '0' && digit != min_num.chars().nth(0).unwrap() {
                    min_num = Self::replace(min_num.as_str(), digit, '0');
                    break;
                }
            }
        }

        max_num.parse::<i32>().unwrap() - min_num.parse::<i32>().unwrap()
    }

    fn replace(s: &str, x: char, y: char) -> String {
        s.chars().map(|c| if c == x { y } else { c }).collect()
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = vec![Input { num: 555 }, Input { num: 9 }];

    for input in inputs {
        let result = Solution::max_diff(input.num);
        println!("{:?}", result);
    }
}
