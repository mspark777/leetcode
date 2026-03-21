struct Solution;

impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut num = n;
        let mut counts = [-1; 10];

        while num > 0 {
            let digit = num % 10;
            let idx = digit as usize;
            counts[idx] += 1;
            num /= 10;
        }

        let mut result = 0;
        let mut min_frequency = i32::MAX;
        for (i, c) in counts.into_iter().enumerate() {
            if c == -1 {
                continue;
            }

            let i = i as i32;
            let c = c + 1;
            if c == 1 {
                return i;
            }

            if c < min_frequency {
                min_frequency = c;
                result = i;
            }
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 1553322 }, Input { n: 723344511 }];

    for input in inputs {
        let result = Solution::get_least_frequent_digit(input.n);
        println!("{:?}", result);
    }
}
