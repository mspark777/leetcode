struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().copied().filter(|&h| h >= target).count() as i32
    }
}

struct Input {
    hours: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            hours: [0, 1, 2, 3, 4].to_vec(),
            target: 2,
        },
        Input {
            hours: [5, 1, 4, 2, 2].to_vec(),
            target: 6,
        },
    ];

    for input in inputs {
        let result = Solution::number_of_employees_who_met_target(input.hours, input.target);
        println!("{:?}", result);
    }
}
