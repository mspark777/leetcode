use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: HashSet<u8> = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']
            .iter()
            .cloned()
            .collect();

        let mut first = 0;
        let mut second = 0;

        let bytes = s.as_bytes();
        let mut i = 0usize;
        let mut j = bytes.len() / 2;
        while j < bytes.len() {
            if vowels.contains(&bytes[i]) {
                first += 1;
            }

            if vowels.contains(&bytes[j]) {
                second += 1;
            }

            i += 1;
            j += 1;
        }

        return first == second;
    }
}

fn main() {
    let inputs = ["book", "textbook"];

    for s in inputs {
        let result = Solution::halves_are_alike(s.to_string());
        println!("{result}");
    }
}
