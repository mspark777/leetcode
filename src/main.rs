struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut ans = HashSet::new();
        let mut row = 1;

        while row < bound {
            let mut col = 1;

            while row + col <= bound {
                ans.insert(row + col);
                if y == 1 {
                    break;
                };
                col *= y;
            }

            if x == 1 {
                break;
            };
            row *= x;
        }

        ans.into_iter().collect()
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
