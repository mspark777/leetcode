use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut result = Vec::<String>::with_capacity(words.len());
        for word in words.iter() {
            if Self::find_pattern(word, &pattern) {
                result.push(word.clone());
            }
        }

        result
    }

    fn find_pattern(word: &String, pattern: &String) -> bool {
        let mut wmap = HashMap::<char, char>::new();
        let mut pmap = HashMap::<char, char>::new();

        let mut pchars = pattern.chars();
        for wc in word.chars() {
            if let Some(pc) = pchars.next() {
                if !wmap.contains_key(&wc) {
                    wmap.insert(wc, pc);
                }

                if !pmap.contains_key(&pc) {
                    pmap.insert(pc, wc);
                }

                let w = wmap.get(&wc).unwrap();
                let p = pmap.get(&pc).unwrap();

                if *w != pc {
                    return false;
                }

                if *p != wc {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
