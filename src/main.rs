struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        for i in 1..=(n / 2) {
            let mut j = 1;
            while j.max(i) <= (n - i - j) {
                if Self::is_valid(&num, i, j) {
                    return true;
                } else {
                    j += 1;
                }
            }
        }

        false
    }

    fn is_valid(num: &str, i: usize, j: usize) -> bool {
        if (num.chars().nth(0).unwrap() == '0') && (i > 1) {
            return false;
        }

        if (num.chars().nth(i).unwrap() == '0') && (j > 1) {
            return false;
        }

        let mut x1 = num
            .chars()
            .take(i)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let mut x2 = num
            .chars()
            .skip(i)
            .take(j)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        let mut start = i + j;
        while start != num.len() {
            x2 += x1;
            x1 = x2 - x1;
            let sum = x2.to_string();
            if !num[start..].starts_with(&sum) {
                return false;
            }

            start += sum.len();
        }

        true
    }
}

struct Input {
    num: String,
}

fn main() {
    let inputs = [
        Input {
            num: "112358".to_string(),
        },
        Input {
            num: "199100199".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_additive_number(input.num);
        println!("{:?}", result);
    }
}
