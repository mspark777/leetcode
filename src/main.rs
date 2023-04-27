use utils::Solution;

mod utils;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        return f64::sqrt(n as f64) as i32;
    }
}

fn main() {
    let inputs = [3, 0, 1];

    for n in inputs {
        let result = Solution::bulb_switch(n);
        println!("{result}");
    }
}
