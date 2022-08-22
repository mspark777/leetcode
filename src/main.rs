struct Solution {}
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        return (n > 0) && ((n & (n - 1)) == 0) && ((n & 0x55555555) != 0);
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 16 }, Input { n: 5 }, Input { n: 1 }];

    for input in inputs {
        let n = input.n;
        let result = Solution::is_power_of_four(n);
        println!("{:?}", result);
    }
}
