struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();
        let mut dp = vec![0; bytes2.len() + 1];

        for b1 in bytes1.iter() {
            let mut prev = dp[0];
            for (j, b2) in bytes2.iter().enumerate() {
                let next = j + 1;
                let val = dp[next].max(if b1 == b2 { prev + 1 } else { dp[j] });
                prev = dp[next];
                dp[next] = val;
            }
        }

        let result = bytes1.len() + bytes2.len() - (2 * dp[bytes2.len()]);
        result as i32
    }
}

fn main() {
    let inputs = [("sea", "eat"), ("leetcode", "etco"), ("ab", "a")];

    for input in inputs {
        let result = Solution::min_distance(String::from(input.0), String::from(input.1));
        println!("{result:?}");
    }
}
