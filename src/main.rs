use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut freq1 = HashMap::<u8, i32>::new();
        let mut freq2 = HashMap::<u8, i32>::new();
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();
        for i in 0..bytes1.len() {
            let ch1 = bytes1[i];
            let ch2 = bytes2[i];
            *freq1.entry(ch1).or_insert(0) += 1;
            *freq2.entry(ch2).or_insert(0) += 1;
        }

        if freq1.len() != freq2.len() {
            return false;
        }

        let keys1: HashSet<u8> = freq1.keys().cloned().collect();
        let keys2: HashSet<u8> = freq2.keys().cloned().collect();
        if keys1 != keys2 {
            return false;
        }

        let mut counts1: Vec<i32> = freq1.values().cloned().collect();
        let mut counts2: Vec<i32> = freq2.values().cloned().collect();

        counts1.sort_unstable_by_key(|k| -k);
        counts2.sort_unstable_by_key(|k| -k);

        for i in 0..counts1.len() {
            if counts1[i] != counts2[i] {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let inputs = [("abc", "bca"), ("a", "aa"), ("cabbba", "abbccc")];

    for (word1, word2) in inputs {
        let result = Solution::close_strings(word1.to_string(), word2.to_string());
        println!("{result}");
    }
}
