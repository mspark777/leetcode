use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        return Self::transform(&s) == Self::transform(&t);
    }

    fn transform(s: &String) -> String {
        let bytes = s.as_bytes();
        let bytes_len = bytes.len();
        let mut index_mapping = HashMap::<u8, usize>::with_capacity(bytes_len);
        let mut result = Vec::<String>::with_capacity(bytes_len);

        for i in 0..bytes_len {
            let ch = bytes[i];

            if let Some(idx) = index_mapping.get(&ch) {
                result.push(idx.to_string());
            } else {
                index_mapping.insert(ch, i);
                result.push(i.to_string());
            }
        }

        result.join(" ")
    }
}

struct Input {
    s: &'static str,
    t: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { s: "egg", t: "add" },
        Input { s: "foo", t: "bar" },
        Input {
            s: "paper",
            t: "title",
        },
    ];

    for input in inputs {
        let s = input.s.to_string();
        let t = input.t.to_string();
        let result = Solution::is_isomorphic(s, t);
        println!("{:?}", result);
    }
}
