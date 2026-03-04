struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for (i, p) in nums.iter().copied().enumerate() {
            let mut p = p as i64;
            let mut gcd = p;
            let mut lcm = p;
            let mut len = 1;

            for n in nums.iter().skip(i + 1).copied() {
                let n = n as i64;
                gcd = Self::gcd(gcd, n);
                p *= n;
                lcm = Self::lcm(lcm, n);

                if p != (gcd * lcm) {
                    break;
                }

                len += 1;
                result = result.max(len);
            }
        }

        result
    }

    fn gcd(a: i64, b: i64) -> i64 {
        match b {
            0 => a,
            _ => Self::gcd(b, a % b),
        }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        (a / Self::gcd(a, b)) * b
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 1, 2, 1, 1, 1].to_vec(),
        },
        Input {
            nums: [2, 3, 4, 5, 6].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 1, 4, 5, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_length(input.nums);
        println!("{:?}", result);
    }
}
