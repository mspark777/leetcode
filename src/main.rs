use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut words = words;
        words.sort_unstable_by_key(|k| k.len());

        let mut dp = HashMap::<&String, i32>::new();
        for word in words.iter() {
            let mut cur_len = 1;
            for i in 0..word.len() {
                let mut predecessor = word.clone();
                predecessor.remove(i);
                if let Some(pre_len) = dp.get(&predecessor) {
                    cur_len = cur_len.max(pre_len + 1);
                }
            }

            dp.insert(word, cur_len);
            result = result.max(cur_len);
        }

        result
    }
}

fn main() {
    let inputs = [
        vec!["a", "b", "ba", "bca", "bda", "bdca"],
        vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"],
        vec!["abcd", "dbqca"],
    ];

    for input in inputs {
        let result = Solution::longest_str_chain(input.iter().map(|s| String::from(*s)).collect());
        println!("{result:?}");
    }
}
