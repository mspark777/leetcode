struct Solution;

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut result = String::new();
        let chars = s.chars().collect::<Vec<char>>();
        let mut marks = vec![-1; chars.len()];
        for (idx, i) in indices.iter().enumerate() {
            let source = sources[idx].chars().collect::<Vec<char>>();
            let e = *i as usize + source.len();
            if e > chars.len() {
                continue;
            }
            if source == &chars[*i as usize..e] {
                for idx2 in *i as usize..e {
                    marks[idx2] = idx as i32;
                }
            }
        }

        let mut idx = 0;

        while idx < marks.len() {
            if marks[idx] == -1 {
                result.push(chars[idx]);
                idx += 1;
            } else {
                let prev = marks[idx];
                while idx < marks.len() - 1 {
                    if prev != marks[idx + 1] {
                        break;
                    } else {
                        idx += 1;
                    }
                }

                result += targets[prev as usize].as_str();
                idx += 1;
            }
        }
        result
    }
}

struct Input {
    fronts: Vec<i32>,
    backs: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        s: "(123)".to_string(),
    }];

    for input in inputs {
        let result = Solution::ambiguous_coordinates(input.s);
        println!("{:?}", result);
    }
}
