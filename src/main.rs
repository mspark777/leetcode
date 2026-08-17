struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut result = String::new();
        let mut shift = shifts.iter().copied().fold(0, |acc, cur| (acc + cur) % 26);

        for (i, ch) in s.char_indices() {
            const A: i32 = b'a' as i32;
            let idx = (ch as i32) - A;
            let shifted = A + ((idx + shift) % 26);

            result.push(shifted as u8 as char);
            shift = (shift - shifts[i]).rem_euclid(26);
        }

        result
    }
}

struct Input {
    s: String,
    shifts: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        s: "bad".to_string(),
        shifts: [10, 20, 30].to_vec(),
    }];

    for input in inputs {
        let result = Solution::shifting_letters(input.s, input.shifts);
        println!("{:?}", result);
    }
}
