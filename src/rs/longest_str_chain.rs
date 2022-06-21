use std::collections::HashMap;

pub struct LongestStrChain {}
impl LongestStrChain {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut dp = HashMap::<String, i32>::new();
        for word in words.iter() {
            dp.insert(word.clone(), -1);
        }

        for word in words.iter() {
            result = result.max(Self::dfs(word.clone(), &mut dp))
        }

        result
    }

    fn dfs(word: String, dp: &mut HashMap<String, i32>) -> i32 {
        if let Some(memo) = dp.get(&word) {
            if *memo > 0 {
                return *memo;
            }
        }

        let mut result = 1;
        for i in 0..word.len() {
            let predecessor = String::new() + &word[0..i] + &word[i + 1..];
            if let Some(_) = dp.get(&predecessor) {
                result = result.max(1 + Self::dfs(predecessor, dp));
            }
        }

        dp.insert(word, result);
        result
    }

    pub fn longest_str_chain1(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut words = words;
        words.sort_unstable_by_key(|k| k.len());

        let mut dp = HashMap::<&String, i32>::new();
        for word in words.iter() {
            let mut cur_len = 1;
            for i in 0..word.len() {
                let predecessor = String::new() + &word[0..i] + &word[i + 1..];
                cur_len = if let Some(pre_len) = dp.get(&predecessor) {
                    cur_len.max(pre_len + 1)
                } else {
                    cur_len
                };
            }

            dp.insert(word, cur_len);
            result = result.max(cur_len);
        }

        result
    }

    pub fn longest_str_chain2(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut words = words;
        words.sort_unstable_by_key(|k| k.len());

        let mut dp = HashMap::<&String, i32>::new();
        for word in words.iter() {
            let mut cur_len = 1;
            for i in 0..word.len() {
                let mut predecessor = word.clone();
                predecessor.remove(i);
                cur_len = if let Some(pre_len) = dp.get(&predecessor) {
                    cur_len.max(pre_len + 1)
                } else {
                    cur_len
                };
            }

            dp.insert(word, cur_len);
            result = result.max(cur_len);
        }

        result
    }

    pub fn longest_str_chain3(words: Vec<String>) -> i32 {
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
