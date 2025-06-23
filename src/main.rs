struct Solution {}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let k = k as i64;
        let mut digit = [0; 100];
        let mut left = 1;
        let mut count = 0;
        let mut result = 0i64;
        while count < n {
            let right = left * 10;
            for op in 0..2 {
                for i in left..right {
                    if count == n {
                        break;
                    }

                    let mut combined = i as i64;
                    let mut x = if op == 0 { i / 10 } else { i };
                    while x > 0 {
                        combined = combined * 10 + (x % 10) as i64;
                        x /= 10;
                    }
                    if Self::is_palindrome(combined, k, &mut digit) {
                        count += 1;
                        result += combined;
                    }
                }
            }
            left = right;
        }

        result
    }

    fn is_palindrome(mut x: i64, k: i64, digit: &mut [i32]) -> bool {
        let mut j = 0usize;
        for i in 0..digit.len() {
            if x <= 0 {
                break;
            } else {
                digit[i] = (x % k) as i32;
                x /= k;
                j = i;
            }
        }

        let mut i = 0usize;
        while i < j {
            if digit[i] != digit[j] {
                return false;
            }

            i += 1;
            j -= if j > 1 { 1 } else { 0 };
        }

        true
    }
}

struct Input {
    k: i32,
    n: i32,
}

fn main() {
    let inputs = vec![
        Input { k: 2, n: 5 },
        Input { k: 3, n: 7 },
        Input { k: 7, n: 17 },
    ];

    for input in inputs {
        let result = Solution::k_mirror(input.k, input.n);
        println!("{:?}", result);
    }
}
