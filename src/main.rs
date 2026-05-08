struct Solution;

#[derive(Eq, PartialEq)]
enum OpType {
    Left,
    Right,
}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut n = n;
        let mut diff = 1;
        let mut result = 1;
        let mut op_type = OpType::Right;
        while n != 1 {
            result = match op_type {
                OpType::Right => result + diff,
                OpType::Left if (n & 1) == 1 => result + diff,
                _ => result,
            };

            n /= 2;
            op_type = match op_type {
                OpType::Left => OpType::Right,
                OpType::Right => OpType::Left,
            };
            diff *= 2;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 9 }, Input { n: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::last_remaining(input.n);
        println!("{:?}", result);
    }
}
