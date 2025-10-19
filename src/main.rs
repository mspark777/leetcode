struct Solution {}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut mask = 0u64;
        for ch in s.chars() {
            let code = ch as u64;
            let shift = code - 'A' as u64;
            mask |= 1 << shift;
        }

        mask &= mask >> 32;
        if mask == 0 {
            return String::new();
        }

        let msb_pos = (63 - mask.leading_zeros()) as u8;
        return ((msb_pos + 'A' as u8) as char).to_string();
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "lEeTcOdE".to_string(),
        },
        Input {
            s: "arRAzFif".to_string(),
        },
        Input {
            s: "AbCdEfGhIjK".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::greatest_letter(input.s);
        println!("{:?}", result);
    }
}
