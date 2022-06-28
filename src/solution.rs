pub struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut frequency = [0; 26];
        for ch in s.as_bytes() {
            let code = *ch - b'a';
            let i = code as usize;
            frequency[i] += 1;
        }

        frequency.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = 0;
        let mut max = s.len() as i32;

        for i in 0..frequency.len() {
            let f = frequency[i];
            if f < 1 {
                break;
            }

            if f > max {
                result += f - max;
                frequency[i] = max;
            }

            max = 0.max(frequency[i] - 1);
        }

        result
    }
}
