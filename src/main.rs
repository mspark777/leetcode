mod utils;

use utils::Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let sbytes = s.as_bytes();
        let gbytes = goal.as_bytes();
        let slen = sbytes.len();
        let glen = gbytes.len();
        if slen != glen {
            return false;
        }

        if s == goal {
            let mut counts = vec![0; 26];
            for &c in sbytes.iter() {
                let code = (c - b'a') as usize;
                if counts[code] == 1 {
                    return true;
                } else {
                    counts[code] += 1;
                }
            }
        }

        let mut first = slen;
        let mut second = slen;
        for i in 0..sbytes.len() {
            let c = sbytes[i];
            let g = gbytes[i];
            if c == g {
                continue;
            }

            if first == sbytes.len() {
                first = i;
            } else if second == sbytes.len() {
                second = i;
            } else {
                return false;
            }
        }

        if first == slen {
            return false;
        } else if second == slen {
            return false;
        }

        if sbytes[first] != gbytes[second] {
            return false;
        } else if sbytes[second] != gbytes[first] {
            return false;
        }

        return true;
    }
}

fn main() {
    let inputs = [("ab", "ba"), ("ab", "ab"), ("aa", "aa")];

    for (s, goal) in inputs {
        let result = Solution::buddy_strings(s.to_string(), goal.to_string());
        println!("{result}");
    }
}
