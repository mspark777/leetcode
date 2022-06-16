struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut s = s;
        let bytes = s.as_bytes();
        let slen = bytes.len() as i32;
        if slen < 2 {
            return s;
        } else if slen == 2 {
            return if bytes[0] == bytes[1] {
                s
            } else {
                s.remove(0).to_string()
            };
        }

        let mut maxlen = 0;
        let mut start = 0;
        for i in 1..(slen - 1) {
            let code = bytes[i as usize];
            let mut low = i - 1;

            while (low > -1) && (bytes[low as usize] == code) {
                low -= 1;
            }

            let mut high = i + 1;
            while (high < slen) && (bytes[high as usize] == code) {
                high += 1;
            }

            while (low > -1) && (high < slen) {
                if bytes[low as usize] == bytes[high as usize] {
                    low -= 1;
                    high += 1;
                } else {
                    break;
                }
            }

            let curlen = high - low - 1;
            if maxlen < curlen {
                maxlen = curlen;
                start = low + 1;
            }
        }

        let begin = start as usize;
        let end = (start + maxlen) as usize;
        let sub = &bytes[begin..end];
        String::from_utf8(sub.to_vec()).expect("oops")
    }
}

fn main() {
    let inputs = ["babad", "cbbd", "ac", "aba"];

    for input in inputs {
        let result = Solution::longest_palindrome(String::from(input));
        println!("{result:?}");
    }
}
