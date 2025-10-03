struct Solution {}

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut result = 0;
        let mut num1 = num1;
        let mut num2 = num2;

        while (num1 != 0) && (num2 != 0) {
            if num1 >= num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            result += 1;
        }

        result
    }
}

struct Input {
    num1: i32,
    num2: i32,
}

fn main() {
    let inputs = [Input { num1: 2, num2: 3 }, Input { num1: 10, num2: 10 }];

    for input in inputs {
        let result = Solution::count_operations(input.num1, input.num2);
        println!("{:?}", result);
    }
}
