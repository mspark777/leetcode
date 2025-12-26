struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let (n, ones) = s.chars().fold((0usize, 0usize), |(n, count), ch| match ch {
            '1' => (n + 1, count + 1),
            _ => (n + 1, count),
        });
        let mut result = String::with_capacity(n);

        for _ in 1..ones {
            result.push('1');
        }

        let zeros = n - ones;
        for _ in 0..zeros {
            result.push('0');
        }

        result.push('1');
        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "010".to_string(),
        },
        Input {
            s: "0101".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::maximum_odd_binary_number(input.s);
        println!("{:?}", result);
    }
}
