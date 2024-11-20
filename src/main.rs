struct Solution {}

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut counts = vec![0; 3];
        let chars: Vec<char> = s.chars().collect();

        for &ch in chars.iter() {
            let idx = Self::char_to_idx(ch);
            counts[idx] += 1;
        }

        for &count in counts.iter() {
            if count < k {
                return -1;
            }
        }

        let mut window = vec![0; 3];
        let mut left = 0usize;
        let mut max_window = 0usize;

        for right in 0..chars.len() {
            window[Self::char_to_idx(chars[right])] += 1;

            while left <= right
                && (counts[0] - window[0] < k
                    || counts[1] - window[1] < k
                    || counts[2] - window[2] < k)
            {
                window[Self::char_to_idx(chars[left])] -= 1;
                left += 1;
            }

            max_window = max_window.max(right + 1 - left);
        }

        return (chars.len() - max_window) as i32;
    }

    fn char_to_idx(ch: char) -> usize {
        let code = ch as u8;
        let a = b'a';

        return (code - a) as usize;
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            s: "aabaaaacaabc",
            k: 2,
        },
        Input { s: "a", k: 1 },
    ];

    for input in inputs {
        let result = Solution::take_characters(input.s.to_string(), input.k);
        println!("{result}");
    }
}
