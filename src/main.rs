struct Solution {}
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        const ACODE: u32 = b'a' as u32;
        let mut bits = 0u32;

        for ch in sentence.chars() {
            let code = ch as u32;
            let offset = code - ACODE;
            let bit = 1 << offset;

            bits |= bit;
        }

        return bits == 0x03ffffff;
    }
}

fn main() {
    let inputs = ["thequickbrownfoxjumpsoverthelazydog", "leetcode"];

    for input in inputs {
        let result = Solution::check_if_pangram(input.to_string());
        println!("{result}");
    }
}
