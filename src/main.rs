struct Solution {}
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num != 0 {
            1 + ((num - 1) % 9)
        } else {
            0
        }
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { num: 38 }, Input { num: 0 }];

    for input in inputs.iter() {
        let num = input.num;
        let result = Solution::add_digits(num);
        println!("{result}");
    }
}
