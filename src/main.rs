struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let target_x = target[0];
        let target_y = target[1];
        let distance = target_x.abs() + target_y.abs();

        for ghost in ghosts {
            let d = (target_x - ghost[0]).abs() + (target_y - ghost[1]).abs();
            if distance >= d {
                return false;
            }
        }

        true
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 10 }, Input { n: 1 }, Input { n: 2 }];

    for input in inputs {
        let result = Solution::rotated_digits(input.n);
        println!("{:?}", result);
    }
}
