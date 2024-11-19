struct Solution {}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut number_of_lines = 1;
        let mut width_of_pixels = 0;

        for ch in s.chars() {
            let width = widths[Self::to_idx(ch)];
            let next_width = width_of_pixels + width;
            if next_width > 100 {
                width_of_pixels = width;
                number_of_lines += 1;
            } else {
                width_of_pixels = next_width;
            }
        }

        return vec![number_of_lines, width_of_pixels];
    }

    fn to_idx(ch: char) -> usize {
        let code = ch as u8;
        let a = b'a';

        return (code - a) as usize;
    }
}

struct Input {
    widths: Vec<i32>,
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            widths: vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10,
            ],
            s: "abcdefghijklmnopqrstuvwxyz",
        },
        Input {
            widths: vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10,
            ],
            s: "bbbcccdddaaa",
        },
    ];

    for input in inputs {
        let result = Solution::number_of_lines(input.widths, input.s.to_string());
        println!("{result:?}");
    }
}
