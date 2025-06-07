struct Solution {}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut counts: Vec<Vec<usize>> = vec![vec![]; 26];
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut indices = Vec::<usize>::new();

        for (i, ch) in chars.iter().cloned().enumerate() {
            if ch != '*' {
                counts[((ch as u8) - b'a') as usize].push(i);
            } else {
                for j in 0..26 {
                    if let Some(idx) = counts[j].pop() {
                        indices.push(idx);
                        break;
                    }
                }
            }
        }

        for i in indices {
            chars[i] = '*';
        }

        return chars.iter().cloned().filter(|&c| c != '*').collect();
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "aaba*" }, Input { s: "abc" }];

    for input in inputs {
        let result = Solution::clear_stars(input.s.to_string());
        println!("{:?}", result);
    }
}
