struct Solution {}

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

struct Input {
    arrival_time: i32,
    delayed_time: i32,
}

fn main() {
    let inputs = [
        Input {
            arrival_time: 15,
            delayed_time: 5,
        },
        Input {
            arrival_time: 13,
            delayed_time: 11,
        },
    ];

    for input in inputs {
        let result = Solution::find_delayed_arrival_time(input.arrival_time, input.delayed_time);
        println!("{:?}", result);
    }
}
