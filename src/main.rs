struct Solution {}
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n > 0 {
            (n & (n - 1)) == 0
        } else {
            false
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 1 }, Input { n: 16 }, Input { n: 3 }];

    for input in inputs {
        let n = input.n;
        let result = Solution::is_power_of_two(n);
        println!("{:?}", result);
    }
}
