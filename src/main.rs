struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        Self::height(red, blue).max(Self::height(blue, red))
    }

    fn height(color1: i32, color2: i32) -> i32 {
        let mut row = 1;
        let mut result = 0;
        let mut idx = 0usize;
        let mut colors = [color1, color2];

        while colors[idx] >= row {
            result += 1;
            colors[idx] -= row;
            row += 1;
            idx ^= 1;
        }

        result
    }
}

struct Input {
    red: i32,
    blue: i32,
}

fn main() {
    let inputs = [
        Input { red: 2, blue: 4 },
        Input { red: 2, blue: 1 },
        Input { red: 1, blue: 1 },
        Input { red: 10, blue: 1 },
    ];

    for input in inputs {
        let result = Solution::max_height_of_triangle(input.red, input.blue);
        println!("{:?}", result);
    }
}
