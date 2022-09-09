struct Solution {}
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        while n > 1 {
            if (n % 2) == 0 {
                n /= 2;
            } else if (n % 3) == 0 {
                n /= 3;
            } else if (n % 5) == 0 {
                n /= 5;
            } else {
                return false;
            }
        }

        n == 1
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { n: 6 },
        Input { n: 1 },
        Input { n: 14 },
        Input { n: -2147483648 },
    ];

    for Input { n } in inputs.iter() {
        let result = Solution::is_ugly(*n);
        println!("{result:?}");
    }
}
