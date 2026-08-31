struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut d = (0, 1);
        let mut p = (0, 0);
        for c in instructions.chars() {
            match c {
                'G' => p = (p.0 + d.0, p.1 + d.1),
                'L' => d = (-d.1, d.0),
                'R' => d = (d.1, -d.0),
                _ => {}
            }
        }
        p == (0, 0) || d != (0, 1)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
