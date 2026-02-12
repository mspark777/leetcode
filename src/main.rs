struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut result = 0;
        let mut count = 0;

        for ch in s.chars() {
            match ch {
                'E' => count += 1,
                _ => count -= 1,
            };

            result = result.max(count)
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "EEEEEEE".to_string(),
        },
        Input {
            s: "ELELEEL".to_string(),
        },
        Input {
            s: "ELEELEELLL".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_chairs(input.s);
        println!("{:?}", result);
    }
}
