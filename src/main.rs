mod utils;

use utils::Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut left = 1;
        let mut right = 10000001;
        let mut min_speed = -1;

        while left <= right {
            let mid = (left + right) / 2;
            if Self::time_required(&dist, mid) <= hour {
                min_speed = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return min_speed;
    }

    fn time_required(dist: &Vec<i32>, speed: i32) -> f64 {
        let mut time = 0.0;
        for (i, &d) in dist.iter().enumerate() {
            let t = (d as f64) / (speed as f64);
            if (i + 1) == dist.len() {
                time += t;
            } else {
                time += t.ceil();
            }
        }

        return time;
    }
}

fn main() {
    let inputs = [
        (vec![1, 3, 2], 6.0),
        (vec![1, 3, 2], 2.7),
        (vec![1, 3, 2], 1.9),
        (vec![1, 1, 100000], 2.01),
        (
            vec![
                90, 94, 72, 85, 92, 63, 20, 25, 38, 28, 8, 75, 95, 70, 8, 96, 41, 8, 7, 75, 62, 65,
                68, 21, 8, 66, 11, 24, 9, 77, 9, 87, 31, 52, 16, 73, 32, 75, 77, 6, 80, 11, 54, 85,
                75, 73, 67, 41, 34, 27, 86, 92, 41, 31, 34, 51, 17, 86, 83, 39, 59, 41, 97, 10, 2,
                59, 80, 73, 13, 10, 69, 28, 65, 34, 17, 45, 9,
            ],
            393.18,
        ),
    ];

    for (dist, hour) in inputs {
        let result = Solution::min_speed_on_time(dist, hour);
        println!("{result}");
    }
}
