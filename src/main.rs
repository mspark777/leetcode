mod utils;

use utils::Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        return if b1.len() < b2.len() {
            Self::minimum_delete_sum1(b2, b1)
        } else {
            Self::minimum_delete_sum1(b1, b2)
        };
    }

    fn minimum_delete_sum1(s1: &[u8], s2: &[u8]) -> i32 {
        let s1len = s1.len();
        let s2len = s2.len();

        let mut cur_row = vec![0i32; s2len + 1];
        for i in 1..=s2len {
            let prev = cur_row[i - 1];
            cur_row[i] = prev + (s2[i - 1] as i32);
        }

        for i in 1..=s1len {
            let mut diag = cur_row[0];
            cur_row[0] += s1[i - 1] as i32;

            for j in 1..=s2len {
                let answer = if s1[i - 1] == s2[j - 1] {
                    diag
                } else {
                    let lmin = (s1[i - 1] as i32) + cur_row[j];
                    let rmin = (s2[j - 1] as i32) + cur_row[j - 1];
                    lmin.min(rmin)
                };

                diag = cur_row[j];
                cur_row[j] = answer;
            }
        }

        return cur_row[s2len];
    }
}

fn main() {
    let inputs = [vec!["sea", "eat"], vec!["delete", "leet"]];

    for input in inputs {
        let s1 = input[0].to_string();
        let s2 = input[1].to_string();
        let result = Solution::minimum_delete_sum(s1, s2);
        println!("{result}");
    }
}
