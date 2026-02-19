struct Solution;

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        match x.min(y >> 2) & 1 {
            1 => "Alice".to_string(),
            _ => "Bob".to_string(),
        }
    }
}

struct Input {
    x: i32,
    y: i32,
}

fn main() {
    let inputs = [Input { x: 2, y: 7 }, Input { x: 4, y: 11 }];

    for input in inputs {
        let result = Solution::winning_player(input.x, input.y);
        println!("{:?}", result);
    }
}
