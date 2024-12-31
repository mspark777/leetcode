struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = days.last().unwrap().clone() as usize;
        let mut dp = vec![0; last_day + 1];

        let mut i = 0usize;
        for day in 1..=last_day {
            if (day as i32) < days[i] {
                dp[day] = dp[day - 1];
            } else {
                i += 1;
                dp[day] = (dp[day - 1] + costs[0])
                    .min(dp[Self::safe(day, 7)] + costs[1])
                    .min(dp[Self::safe(day, 30)] + costs[2]);
            }
        }

        return dp[last_day];
    }

    fn safe(left: usize, right: usize) -> usize {
        return if left <= right { 0 } else { left - right };
    }
}

struct Input {
    days: Vec<i32>,
    costs: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            days: vec![1, 4, 6, 7, 8, 20],
            costs: vec![2, 7, 15],
        },
        Input {
            days: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31],
            costs: vec![2, 7, 15],
        },
    ];

    for input in inputs {
        let result = Solution::mincost_tickets(input.days, input.costs);
        println!("{result:?}");
    }
}
