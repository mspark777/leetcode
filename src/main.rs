struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        const SIZE: usize = 26;

        let idx = |c: char| ((c as u8) - b'a') as usize;

        let mut anagram = [0u8; SIZE];
        for c in p.chars() {
            anagram[idx(c)] += 1;
        }

        let mut result = Vec::<i32>::new();
        let mut window = [0u8; SIZE];
        let s = s.chars().collect::<Vec<char>>();
        for right in 0..s.len() {
            window[idx(s[right])] += 1;
            if right >= p.len() - 1 {
                let left = right + 1 - p.len();
                if anagram == window {
                    result.push(left as i32);
                }
                window[idx(s[left])] -= 1;
            }
        }

        result
    }
}

struct Input {
    s: String,
    p: String,
}

fn main() {
    let inputs = [
        Input {
            s: "cbaebabacd".to_string(),
            p: "abc".to_string(),
        },
        Input {
            s: "abab".to_string(),
            p: "ab".to_string(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_anagrams(input.s, input.p);
        println!("{:?}", result);
    }
}
