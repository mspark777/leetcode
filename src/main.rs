mod utils;

use utils::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        const STAR: u8 = b'*';
        const QM: u8 = b'?';
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut si = 0usize;
        let mut pi = 0usize;
        let mut mi = 0usize;
        let mut stari = None as Option<usize>;

        while si < s.len() {
            if (pi < p.len()) && ((p[pi] == QM) || (s[si] == p[pi])) {
                si += 1;
                pi += 1;
            } else if (pi < p.len()) && (p[pi] == STAR) {
                stari = Some(pi);
                mi = si;
                pi += 1;
            } else if let Some(sidx) = stari {
                pi = sidx + 1;
                mi += 1;
                si = mi;
            } else {
                return false;
            }
        }

        while pi < p.len() {
            if p[pi] == STAR {
                pi += 1;
            } else {
                break;
            }
        }

        return pi == p.len();
    }
}

fn main() {
    let inputs = [vec!["aa", "a"], vec!["aa", "*"], vec!["cb", "?a"]];

    for input in inputs {
        let s = input[0].to_string();
        let p = input[1].to_string();
        let result = Solution::is_match(s, p);
        println!("{result}");
    }
}
