struct Solution {}

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let mut freq = [0; 26];
        for ch in s.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }

        let mut candidate: Vec<char> = Vec::new();
        for i in (0..26).rev() {
            if freq[i] >= k {
                candidate.push((b'a' + i as u8) as char);
            }
        }

        let mut q = std::collections::VecDeque::<Vec<char>>::new();
        for ch in candidate.iter().cloned() {
            q.push_back(vec![ch]);
        }
        let mut result = Vec::<char>::new();
        while let Some(curr) = q.pop_front() {
            if curr.len() > result.len() {
                result = curr.clone();
            }

            for ch in candidate.iter().cloned() {
                let mut next = curr.clone();
                next.push(ch);
                if Self::is_k_repeated_subsequence(&s, &next, k) {
                    q.push_back(next);
                }
            }
        }

        result.iter().collect()
    }

    fn is_k_repeated_subsequence(s: &str, t: &Vec<char>, k: i32) -> bool {
        let mut pos = 0;
        let mut matched = 0;
        let m = t.len();
        for ch in s.chars() {
            if ch == t[pos] {
                pos += 1;
                if pos == m {
                    pos = 0;
                    matched += 1;
                    if matched == k {
                        return true;
                    }
                }
            }
        }
        false
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            s: "letsleetcode",
            k: 2,
        },
        Input { s: "bb", k: 2 },
        Input { s: "ab", k: 2 },
    ];

    for input in inputs {
        let result = Solution::longest_subsequence_repeated_k(input.s.to_string(), input.k);
        println!("{:?}", result);
    }
}
