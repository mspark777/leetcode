pub struct HasAllCodes {}
impl HasAllCodes {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let size = 1 << k;
        let mut dp = vec![0u8; size];
        let mut hash = 0;
        let mut count = 0;
        let mask = !size;
        for (i, ch) in s.as_bytes().iter().enumerate() {
            hash <<= 1;
            hash &= mask;
            hash |= if *ch == b'1' { 1 } else { 0 };

            if (i >= (k - 1)) && (dp[hash] == 0) {
                dp[hash] = 1;
                count += 1;
            }
        }

        count == size
    }

    pub fn has_all_codes1(s: String, k: i32) -> bool {
        let size = 1 << k;
        let mut hash = 0;
        let mut count = 0;
        let mask = !size;
        let mut dp = vec![0u8; size];
        let zerocode = b'0';
        let mut i = 0;
        for ch in s.bytes() {
            hash <<= 1;
            hash &= mask;
            hash |= (ch - zerocode) as usize;
            if (i >= (k - 1)) && (dp[hash] == 0) {
                dp[hash] = 1;
                count += 1;
            }
            i += 1;
        }

        count == size
    }
}
