pub struct RemovePalindromeSub {}
impl RemovePalindromeSub {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = 0usize;
        let mut j = bytes.len() - 1;

        while i < j {
            let ci = bytes[i];
            let cj = bytes[j];

            if ci != cj {
                return 2;
            } else {
                i += 1;
                j -= 1;
            }
        }

        1
    }
}
