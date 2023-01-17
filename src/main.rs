struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        if s.is_empty() {
            return true;
        }

        let t = t.as_bytes();
        let mut si = 0usize;

        for &c in t.iter() {
            if s[si] == c {
                si += 1;
            }

            if si == s.len() {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let inputs = [
        ["abc", "ahbgdc"],
        ["axc", "ahbgdc"],
        ["", ""],
        ["", "ahbgdc"],
    ];

    for input in inputs {
        let s = input[0].to_string();
        let t = input[1].to_string();
        let result = Solution::is_subsequence(s, t);
        println!("{result}");
    }
}
