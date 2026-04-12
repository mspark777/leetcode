struct Solution;

impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        match timer {
            0 => "Green".to_string(),
            30 => "Orange".to_string(),
            (31..91) => "Red".to_string(),
            _ => "Invalid".to_string(),
        }
    }
}

struct Input {
    timer: i32,
}

fn main() {
    let inputs = [Input { timer: 60 }, Input { timer: 5 }];

    for input in inputs {
        let result = Solution::traffic_signal(input.timer);
        println!("{:?}", result);
    }
}
