struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut counts = [0; 24];
        let mut result = 0;

        for hour in hours {
            let h = (hour % 24) as usize;
            result += counts[(24 - h) % 24];
            counts[h] += 1;
        }

        result
    }
}

struct Input {
    hours: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            hours: [12, 12, 30, 24, 24].to_vec(),
        },
        Input {
            hours: [72, 48, 24, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_complete_day_pairs(input.hours);
        println!("{:?}", result);
    }
}
