struct Solution {}
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp = vec![0usize];
        let mut result = 0usize;

        for s in arr.iter() {
            let mut n = 0usize;
            let mut dup = 0usize;

            for ch in s.chars() {
                let code = (ch as usize) - (b'a' as usize);
                let shift = 1 << code;
                dup |= n & shift;
                n |= shift;
            }

            if dup > 0 {
                continue;
            }

            for i in (0..dp.len()).rev() {
                let memo = dp[i];
                if (memo & n) != 0 {
                    continue;
                }

                let temp = memo | n;
                dp.push(temp);

                let bits = temp.count_ones() as usize;
                if bits > result {
                    result = bits;
                }
            }
        }

        return result as i32;
    }
}

fn main() {
    let inputs = [
        vec!["un", "iq", "ue"],
        vec!["cha", "r", "act", "ers"],
        vec!["abcdefghijklmnopqrstuvwxyz"],
    ];

    for arr in inputs {
        let result = Solution::max_length(arr.iter().map(|s| s.to_string()).collect());
        println!("{result:?}");
    }
}
