pub struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "")
            .into_bytes();

        let bytelen = bytes.len();
        if bytelen < 1 {
            return true;
        }

        let mut i = 0;
        let mut j = bytes.len() - 1;
        while i < j {
            if bytes[i] != bytes[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}
