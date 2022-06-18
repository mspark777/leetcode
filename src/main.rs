use std::collections::HashMap;

struct WordFilter {
    substirngs: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut substirngs = HashMap::<String, i32>::new();

        for i in 0..words.len() {
            let word = &words[i];
            for j in 1..=word.len() {
                let prefix = &word[0..j];
                for k in 0..word.len() {
                    let suffix = &word[k..word.len()];
                    let key = format!("{prefix}#{suffix}");
                    substirngs.insert(key, i as i32);
                }
            }
        }

        WordFilter { substirngs }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let key = format!("{prefix}#{suffix}");
        if let Some(i) = self.substirngs.get(&key) {
            *i
        } else {
            -1
        }
    }
}

fn main() {
    let inputs = [[vec!["apple"], vec!["a", "e"]]];

    for input in inputs {
        let filter = WordFilter::new(input[0].iter().map(|s| String::from(*s)).collect());
        let result = filter.f(String::from(input[1][0]), String::from(input[1][1]));
        println!("{result:?}");
    }
}
