struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let sum = num
            .chars()
            .map(|ch| (ch as i32) - ('0' as i32))
            .enumerate()
            .fold(0, |acc, (i, ch)| match i & 1 {
                1 => acc + ch,
                _ => acc - ch,
            });

        sum == 0
    }
}

struct Input {
    num: String,
}

fn main() {
    let inputs = [
        Input {
            num: "1234".to_string(),
        },
        Input {
            num: "24123".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_balanced(input.num);
        println!("{:?}", result);
    }
}
