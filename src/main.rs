mod utils;

use utils::Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut seens = vec![-1; 26];
        let mut count = 1;
        let mut substart = 0;

        const ACODE: usize = 'a' as usize;
        for (i, ch) in s.chars().enumerate() {
            let code = (ch as usize) - ACODE;
            if seens[code] >= substart {
                count += 1;
                substart = i as i32;
            }
            seens[code] = i as i32;
        }

        return count;
    }
}

fn main() {
    let inputs = ["abacaba".to_string(), "ssssss".to_string()];

    for s in inputs {
        let result = Solution::partition_string(s);
        println!("{result}");
    }
}
