struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut jewel_flags = vec![false; ((b'z' - b'a') as usize) * 2 + 2];

        for jewel in jewels.chars() {
            let idx = Self::to_char_index(jewel);
            jewel_flags[idx] = true;
        }

        let mut result = 0;
        for stone in stones.chars() {
            let idx = Self::to_char_index(stone);
            result += if jewel_flags[idx] { 1 } else { 0 };
        }

        return result;
    }

    fn to_char_index(ch: char) -> usize {
        let lower_a = b'a';
        let upper_a = b'A';
        let upper_z = b'Z';
        let code = ch as u8;
        return if code <= upper_z {
            code + 26 - upper_a
        } else {
            code - lower_a
        } as usize;
    }
}

struct Input {
    jewels: &'static str,
    stones: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            jewels: "aA",
            stones: "aAAbbbb",
        },
        Input {
            jewels: "z",
            stones: "ZZ",
        },
    ];

    for input in inputs {
        let result =
            Solution::num_jewels_in_stones(input.jewels.to_string(), input.stones.to_string());
        println!("{result}");
    }
}
