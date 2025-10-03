struct Solution {}

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let temp = num;
        let mut sum = 0;
        let mut num = num;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        match sum & 1 {
            0 => temp / 2,
            _ => (temp - 1) / 2,
        }
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 4 }, Input { num: 30 }];

    for input in inputs {
        let result = Solution::count_even(input.num);
        println!("{:?}", result);
    }
}
