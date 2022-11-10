struct Solution {}
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut result = Vec::<u8>::new();

        for c in s.as_bytes() {
            if result.last().unwrap_or(&0).eq(c) {
                result.pop();
            } else {
                result.push(*c);
            }
        }

        return String::from_utf8(result).unwrap();
    }
}

fn main() {
    let inputs = ["abbaca", "azxxzy"];

    for s in inputs {
        let result = Solution::remove_duplicates(s.to_string());
        println!("{result}");
    }
}
