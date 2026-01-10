struct Solution;

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut step = 0;

        for p in battery_percentages {
            if p > step {
                step += 1;
                result += 1;
            }
        }

        result
    }
}

struct Input {
    battery_percentages: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            battery_percentages: [1, 1, 2, 1, 3].to_vec(),
        },
        Input {
            battery_percentages: [0, 1, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_tested_devices(input.battery_percentages);
        println!("{:?}", result);
    }
}
