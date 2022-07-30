const LETTER_COUNT: usize = 26;
const ACODE: usize = 'a' as usize;

pub struct Solution {}
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut counts2 = Self::get_counts(&"".to_owned());
        for word in words2.iter() {
            let counts3 = Self::get_counts(word);
            for i in 0..LETTER_COUNT {
                counts2[i] = counts2[i].max(counts3[i]);
            }
        }

        let mut result = Vec::<String>::new();
        for word in words1.iter() {
            let counts1 = Self::get_counts(word);
            let mut ok = true;
            for i in 0..LETTER_COUNT {
                if counts1[i] < counts2[i] {
                    ok = false;
                    break;
                }
            }

            if ok {
                result.push(word.clone());
            }
        }

        result
    }

    fn get_counts(word: &String) -> Vec<i32> {
        let mut counts = vec![0; LETTER_COUNT];
        for ch in word.chars() {
            let c = ch as usize;
            let i = c - ACODE;
            counts[i] += 1;
        }

        counts
    }
}
