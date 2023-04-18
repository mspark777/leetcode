mod utils;

use utils::Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let len1 = word1.len();
        let len2 = word2.len();
        let rlen = len1 + len2;
        let maxlen = len1.max(len2);
        let mut result = vec![0u8; rlen];
        let mut pos = 0usize;
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();

        for i in 0..maxlen {
            if i < len1 {
                result[pos] = bytes1[i];
                pos += 1;
            }

            if i < len2 {
                result[pos] = bytes2[i];
                pos += 1;
            }
        }

        return String::from_utf8(result).unwrap();
    }
}

fn main() {
    let inputs = [("abc", "pqr"), ("ab", "pqrs"), ("abcd", "pq")];

    for (word1, word2) in inputs {
        let result = Solution::merge_alternately(word1.to_string(), word2.to_string());
        println!("{result}");
    }
}
