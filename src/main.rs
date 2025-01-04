struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let letters = std::collections::HashSet::<char>::from_iter(s.iter().cloned());
        let mut result = 0;

        for letter in letters.iter().cloned() {
            let mut i = s.len();
            let mut j = 0usize;

            for (k, ch) in s.iter().cloned().enumerate() {
                if ch == letter {
                    if i == s.len() {
                        i = k;
                    }

                    j = k;
                }
            }

            let left = i + 1;
            let right = j;
            if left >= right {
                continue;
            }

            let between = std::collections::HashSet::<char>::from_iter(
                s.iter().skip(left).take(right - left).cloned(),
            );

            result += between.len() as i32;
        }

        return result;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "aabca" },
        Input { s: "adc" },
        Input { s: "bbcbaba" },
        Input {
            s: "aywvhbwycmbttdmogwlfosfizqlndfipffbqfxwbgrfdyomuuecllmsrzckiwgelkhgylwobz",
        },
    ];

    for input in inputs {
        let result = Solution::count_palindromic_subsequence(input.s.to_string());
        println!("{result:?}");
    }
}
