struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const ZERO: u8 = b'0';

        let bytes = s.as_bytes();

        if bytes.is_empty() {
            return 0;
        }

        if bytes[0] == ZERO {
            return 0;
        }

        if bytes.len() == 1 {
            return 1;
        }

        let mut d1 = 1;
        let mut d2 = 1;

        for i in 1..bytes.len() {
            let code1 = bytes[i] - ZERO;
            let code0 = ((bytes[i - 1] - ZERO) * 10) + code1;

            let mut n = 0;
            if code1 != 0 {
                n += d1;
            }

            if (10 <= code0) && (code0 <= 26) {
                n += d2;
            }

            d2 = d1;
            d1 = n;
        }

        return d1;
    }
}

fn main() {
    let inputs = vec!["12", "226", "06"];

    for input in inputs.into_iter() {
        let result = Solution::num_decodings(input.to_string());
        println!("{result:?}");
    }
}
