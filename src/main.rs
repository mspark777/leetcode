struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0; 1];
        for c in s.chars() {
            match c {
                '(' => stack.push(0),
                ')' => {
                    let popped = stack.pop();
                    let last = stack.last_mut();
                    if let (Some(popped), Some(last)) = (popped, last) {
                        *last += 1.max(popped * 2);
                    }
                }
                _ => unreachable!(),
            }
        }
        stack[0]
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "()".to_string(),
    }];

    for input in inputs {
        let result = Solution::score_of_parentheses(input.s);
        println!("{:?}", result);
    }
}
