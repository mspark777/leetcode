struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = s.len();
        let max_unique_count = Self::max_unique_letters(&s);
        let mut result = 0i32;

        for curr_unique in 1..=max_unique_count {
            let mut count_map = [0; 26];
            let mut window_start = 0usize;
            let mut window_end = 0usize;
            let mut unique = 0i32;
            let mut count_at_least_k = 0i32;

            while window_end < n {
                if unique <= curr_unique {
                    let idx = (bytes[window_end] - b'a') as usize;
                    if count_map[idx] == 0 {
                        unique += 1;
                    }

                    count_map[idx] += 1;
                    if count_map[idx] == k {
                        count_at_least_k += 1;
                    }

                    window_end += 1;
                } else {
                    let idx = (bytes[window_start] - b'a') as usize;
                    if count_map[idx] == k {
                        count_at_least_k -= 1;
                    }

                    count_map[idx] -= 1;
                    if count_map[idx] == 0 {
                        unique -= 1;
                    }

                    window_start += 1;
                }

                if (unique == curr_unique) && (unique == count_at_least_k) {
                    let curr_len = window_end - window_start;
                    result = result.max(curr_len as i32);
                }
            }
        }

        result
    }

    fn max_unique_letters(s: &str) -> i32 {
        let mut flags = [false; 26];
        let mut max_unique = 0;
        for ch in s.chars() {
            let flag_idx = (ch as usize) - ('a' as usize);
            if !flags[flag_idx] {
                max_unique += 1;
                flags[flag_idx] = true;
            }
        }

        max_unique
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "aaabb".to_string(),
            k: 3,
        },
        Input {
            s: "ababbc".to_string(),
            k: 2,
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::longest_substring(input.s, input.k);
        println!("{:?}", result);
    }
}
