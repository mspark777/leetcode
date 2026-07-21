struct Solution;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let n = n.min(3);

        match presses {
            0 => 1,
            1 if n == 1 => 2,
            1 if n == 2 => 3,
            1 => 4,
            2 if n == 1 => 2,
            2 if n == 2 => 4,
            2 => 7,
            _ if n == 1 => 2,
            _ if n == 2 => 4,
            _ => 8,
        }
    }
}

struct Input {
    n: i32,
    presses: i32,
}

fn main() {
    let inputs = [Input { n: 1, presses: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::flip_lights(input.n, input.presses);
        println!("{:?}", result);
    }
}
