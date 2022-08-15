struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;

        for ch in s.bytes().rev() {
            let num = match ch {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            };
            let temp = num * 4;
            result += if temp < result { -num } else { num };
        }

        result
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { s: "III" },
        Input { s: "LVIII" },
        Input { s: "MCMXCIV" },
    ];

    for input in inputs {
        let s = input.s.to_string();
        let result = Solution::roman_to_int(s);
        println!("{:?}", result);
    }
}
