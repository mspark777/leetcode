struct Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if n <= 3 {
            return 1;
        }

        let n = n as usize;
        let mut s = vec![0u8; n];
        s[0] = 0;
        s[1] = 1;
        s[2] = 1;

        let mut read = 2;
        let mut write = 3;
        let mut next_value = 0;
        let mut result = 1;

        while write < n {
            let repeat = s[read] + 1;
            for _ in 0..repeat {
                if write >= n {
                    break;
                }

                s[write] = next_value;
                write += 1;
                if next_value == 0 {
                    result += 1;
                }
            }

            next_value ^= 1;
            read += 1;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 6 }, Input { n: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::magical_string(input.n);
        println!("{:?}", result);
    }
}
