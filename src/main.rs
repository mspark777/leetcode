struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let equations = equation.as_bytes();
        let len = equations.len();
        let mut tmp = 0;
        let mut lr = 1;
        let mut a = 0;
        let mut b = 0;
        let mut sign = 1;
        let mut calc = 0;

        for (idx, equation) in equations.iter().copied().enumerate() {
            if equation.is_ascii_digit() {
                tmp = tmp * 10 + (equation - b'0') as i32;
                calc += 1;
            }
            if equation == b'x' {
                if tmp == 0 && calc == 0 {
                    a += sign * lr;
                }
                a += sign * lr * tmp;
                calc = 0;
                tmp = 0;
            }
            if equation == b'-' || equation == b'+' || equation == b'=' || idx == len - 1 {
                b += sign * lr * tmp;
                calc = 0;
                tmp = 0;
                if equation == b'-' {
                    sign = -1;
                } else if equation == b'+' {
                    sign = 1;
                } else if equation == b'=' {
                    lr = -1;
                    sign = 1;
                }
            }
        }

        if a != 0 {
            format!("x={}", -b / a)
        } else if b == 0 {
            "Infinite solutions".to_string()
        } else {
            "No solution".to_string()
        }
    }
}

struct Input {
    equation: String,
}

fn main() {
    let inputs = [Input {
        equation: "x+5-3+x=6+x-2".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::solve_equation(input.equation);
        println!("{:?}", result);
    }
}
