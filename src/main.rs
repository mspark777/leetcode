struct Solution {}

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let part = part.chars().collect::<Vec<char>>();
        let kmp_lps = Self::compute_longest_prefix_suffix(&part);
        let mut char_stack = Vec::<char>::new();
        let mut pattern_indexes = vec![0usize; s.len() + 1];

        let mut str_index = 0usize;
        let mut pattern_index = 0usize;
        while str_index < s.len() {
            let current_char = s[str_index];
            char_stack.push(current_char);

            if current_char == part[pattern_index] {
                pattern_index += 1;
                pattern_indexes[char_stack.len()] = pattern_index;
                if pattern_index == part.len() {
                    let mut remaining_length = part.len();
                    while remaining_length > 0 {
                        char_stack.pop();
                        remaining_length -= 1;
                    }

                    pattern_index = if char_stack.is_empty() {
                        0
                    } else {
                        pattern_indexes[char_stack.len()]
                    };
                }
            } else {
                if pattern_index > 0 {
                    str_index -= 1;
                    pattern_index = kmp_lps[pattern_index - 1];
                    char_stack.pop();
                } else {
                    pattern_indexes[char_stack.len()] = 0;
                }
            }

            str_index += 1;
        }

        let mut result = Vec::<char>::with_capacity(char_stack.len());
        while let Some(ch) = char_stack.pop() {
            result.push(ch);
        }

        return result.iter().rev().collect();
    }

    fn compute_longest_prefix_suffix(pattern: &Vec<char>) -> Vec<usize> {
        let mut lps = vec![0usize; pattern.len()];

        let mut current = 1usize;
        let mut prefix_length = 0usize;
        while current < pattern.len() {
            if pattern[current] == pattern[prefix_length] {
                prefix_length += 1;
                lps[current] = prefix_length;
                current += 1;
            } else if prefix_length != 0 {
                prefix_length = lps[prefix_length - 1];
            } else {
                lps[current] = 0;
                current += 1;
            }
        }

        return lps;
    }
}

struct Input {
    s: &'static str,
    part: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            s: "daabcbaabcbc",
            part: "abc",
        },
        Input {
            s: "axxxxyyyyb",
            part: "xy",
        },
    ];

    for input in inputs {
        let result = Solution::remove_occurrences(input.s.to_string(), input.part.to_string());
        println!("{result:?}");
    }
}
