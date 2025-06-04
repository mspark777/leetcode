struct Solution {}

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let num_friends = num_friends as usize;
        let n = word.len();
        let last = Self::last_substring(&word);
        let m = last.len();
        let len = m.min(n - num_friends + 1);
        return last[..len].to_string();
    }

    fn last_substring(s: &String) -> String {
        let chars = s.chars().collect::<Vec<_>>();
        let n = chars.len();
        let mut i = 0usize;
        let mut j = 1usize;
        while j < n {
            let mut k = 0usize;
            while j + k < n && chars[i + k] == chars[j + k] {
                k += 1;
            }

            if j + k < n && chars[i + k] < chars[j + k] {
                let t = i;
                i = j;
                j = (j + 1).max(t + k + 1);
            } else {
                j = j + k + 1;
            }
        }

        return chars[i..].iter().collect();
    }
}

struct Input {
    word: &'static str,
    num_friends: i32,
}

fn main() {
    let inputs = vec![
        Input {
            word: "dbca",
            num_friends: 2,
        },
        Input {
            word: "gggg",
            num_friends: 4,
        },
    ];

    for input in inputs {
        let result = Solution::answer_string(input.word.to_string(), input.num_friends);
        println!("{:?}", result);
    }
}
