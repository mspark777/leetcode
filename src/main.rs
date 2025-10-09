struct Solution {}

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut chars = s.chars();
        let c1 = chars.next().unwrap() as u8;
        let r1 = chars.next().unwrap() as u8;
        chars.next();
        let c2 = chars.next().unwrap() as u8;
        let r2 = chars.next().unwrap() as u8;

        let n = (c2 - c1 + 1) * (r2 - r1 + 1);
        let mut result = Vec::<String>::with_capacity(n as usize);

        for c in c1..=c2 {
            for r in r1..=r2 {
                let cell = format!("{}{}", (c as char), (r as char));
                result.push(cell);
            }
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
            s: "K1:L2".to_string(),
        },
        Input {
            s: "A1:F1".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::cells_in_range(input.s);
        println!("{:?}", result);
    }
}
