struct Solution {}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut ch_map = std::collections::HashMap::<char, char>::with_capacity(26);
        let mut ch_code = 'a' as u8;

        for key in key.chars() {
            if key == ' ' {
                continue;
            } else if ch_map.contains_key(&key) {
                continue;
            } else {
                let ch = ch_code as char;
                ch_map.insert(key, ch);
                ch_code += 1;
            }
        }

        let mut result = Vec::<char>::with_capacity(message.len());
        for msg in message.chars() {
            if msg == ' ' {
                result.push(msg);
            } else if let Some(&ch) = ch_map.get(&msg) {
                result.push(ch);
            }
        }

        result.iter().collect()
    }
}

struct Input {
    key: String,
    message: String,
}

fn main() {
    let inputs = [
        Input {
            key: "the quick brown fox jumps over the lazy dog".to_string(),
            message: "vkbs bs t suepuv".to_string(),
        },
        Input {
            key: "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
            message: "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::decode_message(input.key, input.message);
        println!("{:?}", result);
    }
}
