struct Solution {}

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string();
        let mut max_num = num;
        for ch in s.chars() {
            if ch == '9' {
                continue;
            } else {
                max_num = s.replace(ch, "9").parse().unwrap();
                break;
            }
        }

        let first = s.chars().next().unwrap();
        let min_num = s.replace(first, "0").parse::<i32>().unwrap();
        max_num - min_num
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = vec![Input { num: 11891 }, Input { num: 90 }];

    for input in inputs {
        let result = Solution::min_max_difference(input.num);
        println!("{:?}", result);
    }
}
