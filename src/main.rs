struct Solution;

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut div = 1;
        let mut result = 0;

        while (num1 >= div) || (num2 >= div) || (num3 >= div) {
            let n1 = (num1 / div) % 10;
            let n2 = (num2 / div) % 10;
            let n3 = (num3 / div) % 10;
            result += div * n1.min(n2).min(n3);
            div *= 10;
        }

        result
    }
}

struct Input {
    num1: i32,
    num2: i32,
    num3: i32,
}

fn main() {
    let inputs = [
        Input {
            num1: 1,
            num2: 10,
            num3: 1000,
        },
        Input {
            num1: 987,
            num2: 879,
            num3: 798,
        },
        Input {
            num1: 1,
            num2: 2,
            num3: 3,
        },
    ];

    for input in inputs {
        let result = Solution::generate_key(input.num1, input.num2, input.num3);
        println!("{:?}", result);
    }
}
