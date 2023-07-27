mod utils;

use utils::Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let n = n as i64;
        let mut sum_power = 0i64;
        for &power in batteries.iter() {
            sum_power += power as i64;
        }

        let mut left = 1i64;
        let mut right = sum_power / n;

        while left < right {
            let target = (left + right + 1) / 2;
            let extra = batteries
                .iter()
                .fold(0, |acc, &power| acc + target.min(power as i64));

            if extra >= (n * target) {
                left = target;
            } else {
                right = target - 1;
            }
        }

        return left;
    }
}

fn main() {
    let inputs = [(2, vec![3, 3, 3]), (2, vec![1, 1, 1, 1])];

    for (n, batteries) in inputs {
        let result = Solution::max_run_time(n, batteries);
        println!("{result}");
    }
}
