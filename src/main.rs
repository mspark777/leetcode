struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        if (x + y) < target {
            return false;
        }

        if (x == target) || (y == target) || ((x + y) == target) {
            return true;
        }

        (target % Self::gcd(x, y)) == 0
    }

    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }

        x
    }
}

struct Input {
    x: i32,
    y: i32,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            x: 3,
            y: 5,
            target: 4,
        },
        Input {
            x: 2,
            y: 6,
            target: 5,
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::can_measure_water(input.x, input.y, input.target);
        println!("{:?}", result);
    }
}
