struct Solution {}
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut sum = 0u8;

        for &i in s.as_bytes() {
            sum ^= i;
        }

        for &i in t.as_bytes() {
            sum ^= i;
        }

        return sum as char;
    }
}

fn main() {
    let inputs = [["abcd", "abcde"], ["", "y"]];

    for input in inputs {
        let result = Solution::find_the_difference(input[0].to_string(), input[1].to_string());
        println!("{result}");
    }
}
