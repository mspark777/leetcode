mod utils;

use utils::Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::<u8>::with_capacity(s.len());
        for &ch in s.as_bytes().iter() {
            if ch == ('*' as u8) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        return String::from_utf8(stack).unwrap();
    }
}

fn main() {
    let inputs = ["leet**cod*e", "erase*****"];

    for s in inputs {
        let result = Solution::remove_stars(s.to_string());
        println!("{result}");
    }
}
