mod utils;

use utils::Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memos = vec![0; days.len()];
        let durations = vec![1, 7, 30];

        return Self::dp(&days, &costs, &durations, &mut memos, 0);
    }

    fn dp(
        days: &Vec<i32>,
        costs: &Vec<i32>,
        durations: &Vec<i32>,
        memos: &mut Vec<i32>,
        i: usize,
    ) -> i32 {
        if i >= days.len() {
            return 0;
        } else if memos[i] != 0 {
            return memos[i];
        }

        let mut result = i32::max_value();
        let mut j = i;

        for (&cost, &duration) in costs.iter().zip(durations.iter()) {
            while j < days.len() {
                let k = days[i] + duration;
                if days[j] < k {
                    j += 1;
                } else {
                    break;
                }
            }

            let recv = Self::dp(days, costs, durations, memos, j) + cost;
            result = result.min(recv);
        }

        memos[i] = result;
        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
    ];

    for (days, costs) in inputs {
        let result = Solution::mincost_tickets(days, costs);
        println!("{result}")
    }
}
