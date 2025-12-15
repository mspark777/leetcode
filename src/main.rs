struct Solution {}

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}

struct Input {
    num: i32,
    t: i32,
}

fn main() {
    let inputs = [Input { num: 5, t: 1 }, Input { num: 3, t: 2 }];

    for input in inputs {
        let result = Solution::the_maximum_achievable_x(input.num, input.t);
        println!("{:?}", result);
    }
}
