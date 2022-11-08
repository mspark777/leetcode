struct Solution {}
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut chars: Vec<u8> = s.bytes().collect();
        let mut j = 0usize;

        for i in 0..chars.len() {
            if j > 0 {
                let cur = chars[i] as i32;
                let next = chars[j - 1] as i32;
                let diff = cur - next;
                if diff.abs() == 32 {
                    j -= 1;
                    continue;
                }
            }

            chars[j] = chars[i];
            j += 1;
        }

        return String::from_utf8(chars.iter().take(j).cloned().collect()).unwrap();
    }
}

fn main() {
    let inputs = ["leEeetcode", "abBAcC", "s"];

    for s in inputs {
        let result = Solution::make_good(s.to_string());
        println!("{result}");
    }
}
