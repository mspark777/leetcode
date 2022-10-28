use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::<Vec<char>, Vec<String>>::with_capacity(strs.len());

        for s in strs.iter() {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();

            groups.entry(key).or_insert(vec![]).push(s.clone());
        }

        return groups.values().cloned().collect();
    }
}

fn main() {
    let inputs = [
        vec!["eat", "tea", "tan", "ate", "nat", "bat"],
        vec![""],
        vec!["a"],
    ];

    for input in inputs {
        let strs = input.iter().map(|s| s.to_string()).collect();
        let result = Solution::group_anagrams(strs);
        println!("{result:?}");
    }
}
