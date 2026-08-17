struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};

enum Slope {
    Center,
    Left(i32),
    Right(i32),
}

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut slope = Slope::Center;
        for (left, right) in arr.windows(2).map(|x| (x[0], x[1])) {
            slope = match (slope, left.cmp(&right)) {
                (Slope::Left(k), Less) => Slope::Left(k + 1),
                (Slope::Right(k), Less) => {
                    result = result.max(k);
                    Slope::Left(2)
                }
                (Slope::Center, Less) => Slope::Left(2),
                (Slope::Right(k), Equal) => {
                    result = result.max(k);
                    Slope::Center
                }
                (Slope::Left(k) | Slope::Right(k), Greater) => {
                    result = result.max(k + 1);
                    Slope::Right(k + 1)
                }
                (_, Equal | Greater) => Slope::Center,
            };
        }
        result
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [2, 1, 4, 7, 3, 2, 5].to_vec(),
    }];

    for input in inputs {
        let result = Solution::longest_mountain(input.arr);
        println!("{:?}", result);
    }
}
