struct Solution {}

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let chars: Vec<char> = word.chars().collect();
        let mut comp = Vec::<char>::with_capacity(chars.len());
        let mut pos = 0usize;

        while pos < chars.len() {
            let mut consecutive_count = 0usize;
            let current = chars[pos];
            while pos < chars.len() && consecutive_count < 9 && chars[pos] == current {
                consecutive_count += 1;
                pos += 1;
            }

            comp.push((('0' as u8) + (consecutive_count as u8)) as char);
            comp.push(current);
        }

        return comp.iter().collect();
    }
}

struct Input {
    word: String,
}

fn main() {
    let inputs = vec![
        Input {
            word: String::from("abcde"),
        },
        Input {
            word: String::from("aaaaaaaaaaaaaabb"),
        },
    ];

    for input in inputs {
        let result = Solution::compressed_string(input.word);
        println!("{result}");
    }
}
