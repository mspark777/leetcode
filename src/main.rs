struct Solution {}
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{str1}{str2}") != format!("{str2}{str1}") {
            return String::new();
        }

        let gcd = Self::gcd(str1.len(), str2.len());
        return str1.chars().take(gcd).collect();
    }

    fn gcd(x: usize, y: usize) -> usize {
        if y == 0 {
            return x;
        }

        return Self::gcd(y, x % y);
    }
}

fn main() {
    let inputs = [
        vec!["ABCABC", "ABC"],
        vec!["ABABAB", "ABAB"],
        vec!["LEET", "CODE"],
    ];

    for input in inputs {
        let str1 = input[0].to_string();
        let str2 = input[1].to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        println!("{result}");
    }
}
