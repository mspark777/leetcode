struct Solution {}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let start = (start - 1).to_string();
        let finish = finish.to_string();
        return Self::calculate(&finish, &s, limit) - Self::calculate(&start, &s, limit);
    }

    fn calculate(x: &str, s: &str, limit: i32) -> i64 {
        if x.len() < s.len() {
            return 0;
        } else if x.len() == s.len() {
            return if x >= s { 1 } else { 0 };
        }

        let suffix = &x[x.len() - s.len()..];
        let mut count = 0i64;
        let pre_len = x.len() - s.len();
        for i in 0..pre_len {
            let digit = x.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            if limit < digit {
                count += ((limit + 1) as i64).pow((pre_len - i) as u32);
                return count;
            }
            count += (digit as i64) * ((limit + 1) as i64).pow((pre_len - 1 - i) as u32);
        }
        if suffix >= s {
            count += 1;
        }

        return count;
    }
}

struct Input {
    start: i64,
    finish: i64,
    limit: i32,
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            start: 1,
            finish: 6000,
            limit: 4,
            s: "124",
        },
        Input {
            start: 15,
            finish: 215,
            limit: 6,
            s: "10",
        },
        Input {
            start: 1000,
            finish: 2000,
            limit: 4,
            s: "3000",
        },
    ];

    for input in inputs {
        let result = Solution::number_of_powerful_int(
            input.start,
            input.finish,
            input.limit,
            input.s.to_string(),
        );
        println!("{result:?}");
    }
}
