struct Solution;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet};

        let (em, lc, vs) = wordlist.into_iter().rev().fold(
            (HashSet::new(), HashMap::new(), HashMap::new()),
            |(mut em, mut lc, mut vs), w| {
                let l = w.to_lowercase();
                em.insert(w.clone());
                vs.insert(Self::rv(&l), w.clone());
                lc.insert(l, w);
                (em, lc, vs)
            },
        );
        queries
            .into_iter()
            .map(|q| {
                if em.contains(&q) {
                    return q;
                }
                let ql = q.to_lowercase();
                if let Some(w) = lc.get(&ql) {
                    return w.to_owned();
                }
                if let Some(w) = vs.get(&Self::rv(&ql)) {
                    return w.to_owned();
                }
                String::new()
            })
            .collect()
    }

    fn rv(s: &String) -> String {
        s.chars()
            .map(|c| match "eiou".contains(c) {
                true => 'a',
                false => c,
            })
            .collect()
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        points: [[1, 2], [2, 1], [1, 0], [0, 1]]
            .map(|v| v.to_vec())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::min_area_free_rect(input.points);
        println!("{:?}", result);
    }
}
