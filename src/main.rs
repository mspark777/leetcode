struct Solution;

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let base16 = Self::to_base(n * n, 16);
        let base36 = Self::to_base(n * n * n, 36);

        format!("{}{}", base16, base36)
    }

    fn to_base(mut num: i32, base: i32) -> String {
        let symbols = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect::<Vec<char>>();
        let mut indexes = Vec::<usize>::with_capacity(((num / base) + 1) as usize);
        while num > 0 {
            let idx = (num % base) as usize;
            indexes.push(idx);
            num /= base;
        }

        indexes.reverse();
        indexes
            .into_iter()
            .map(|idx| symbols[idx].to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 13 }, Input { n: 36 }];

    for input in inputs {
        let result = Solution::concat_hex36(input.n);
        println!("{:?}", result);
    }
}
