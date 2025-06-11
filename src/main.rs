struct Solution {}

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let n = chars.len();
        let mut result = i32::MIN;
        let digits = ['0', '1', '2', '3', '4', '5'];
        for a in digits.iter().cloned() {
            for b in digits.iter().cloned() {
                if a == b {
                    continue;
                }

                let mut best = [i32::MAX; 4];
                let mut cnt_a = 0;
                let mut cnt_b = 0;
                let mut prev_a = 0;
                let mut prev_b = 0;
                let mut left = -1;

                for right in 0..n {
                    if chars[right] == a {
                        cnt_a += 1;
                    }

                    if chars[right] == b {
                        cnt_b += 1;
                    }

                    while (right as i32 - left) >= k && (cnt_b - prev_b) >= 2 {
                        let left_status = Self::get_status(prev_a, prev_b) as usize;
                        best[left_status] = best[left_status].min(prev_a - prev_b);
                        left += 1;

                        if chars[left as usize] == a {
                            prev_a += 1;
                        }

                        if chars[left as usize] == b {
                            prev_b += 1;
                        }
                    }

                    let right_status = Self::get_status(cnt_a, cnt_b) as usize;
                    if best[right_status ^ 0b10] != i32::MAX {
                        result = result.max(cnt_a - cnt_b - best[right_status ^ 0b10]);
                    }
                }
            }
        }

        return result;
    }

    fn get_status(cnt_a: i32, cnt_b: i32) -> i32 {
        return ((cnt_a & 1) << 1) | (cnt_b & 1);
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input { s: "12233", k: 4 },
        Input { s: "1122211", k: 3 },
        Input { s: "110", k: 3 },
    ];

    for input in inputs {
        let result = Solution::max_difference(input.s.to_string(), input.k);
        println!("{:?}", result);
    }
}
