struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lowers = 0u32;
        let mut uppers = 0u32;

        for ch in word.chars() {
            let code = ch as u32;
            if ch.is_ascii_lowercase() {
                let a = 'a' as u32;
                lowers |= 1 << (code - a);
            } else {
                let a = 'A' as u32;
                uppers |= 1 << (code - a);
            }
        }

        let mut result = 0;

        for i in 0..26u32 {
            let mask = 1 << i;
            let l = lowers & mask;
            let u = uppers & mask;
            if (l & u) != 0 {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    word: String,
}

fn main() {
    let inputs = [
        Input {
            word: "aaAbcBC".to_string(),
        },
        Input {
            word: "abc".to_string(),
        },
        Input {
            word: "abBCab".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::number_of_special_chars(input.word);
        println!("{:?}", result);
    }
}
