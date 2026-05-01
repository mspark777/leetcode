struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a & b;
            a ^= b;
            b = temp << 1;
        }

        a
    }
}

struct Input {
    a: i32,
    b: i32,
}

fn main() {
    let inputs = [Input { a: 1, b: 2 }, Input { a: 2, b: 3 }];

    for input in inputs.into_iter() {
        let result = Solution::get_sum(input.a, input.b);
        println!("{:?}", result);
    }
}
