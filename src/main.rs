use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts = HashMap::<u8, i32>::with_capacity(magazine.len());
        for ch in magazine.as_bytes() {
            if let Some(cnt) = counts.get_mut(ch) {
                *cnt += 1;
            } else {
                counts.insert(*ch, 1);
            }
        }

        for ch in ransom_note.as_bytes() {
            if let Some(cnt) = counts.get_mut(ch) {
                if *cnt < 1 {
                    return false;
                } else {
                    *cnt -= 1;
                }
            } else {
                return false;
            }
        }

        true
    }
}

struct Input {
    ransom_note: &'static str,
    magazine: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            ransom_note: "a",
            magazine: "b",
        },
        Input {
            ransom_note: "aa",
            magazine: "ab",
        },
        Input {
            ransom_note: "aa",
            magazine: "aab",
        },
    ];

    for input in inputs.iter() {
        let ransom_note = input.ransom_note.to_string();
        let magazine = input.magazine.to_string();
        let result = Solution::can_construct(ransom_note, magazine);
        println!("{result}");
    }
}
