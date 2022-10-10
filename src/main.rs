struct Solution {}
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut chars: Vec<u8> = palindrome.bytes().collect();
        let slen = chars.len();
        if slen <= 1 {
            return String::new();
        }

        for i in 0..(slen / 2) {
            if chars[i] != b'a' {
                chars[i] = b'a';
                return Self::to_string(&chars);
            }
        }

        chars[slen - 1] = b'b';
        return Self::to_string(&chars);
    }

    fn to_string(chars: &Vec<u8>) -> String {
        return String::from_utf8_lossy(chars.as_slice()).to_string();
    }
}

fn main() {
    let inputs = ["abccba", "a", "aba"];

    for input in inputs {
        let result = Solution::break_palindrome(input.to_string());
        println!("{result}");
    }
}
