pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut step0 = cost[0];
        let mut step1 = cost[1];

        for c in cost.iter().skip(2) {
            let cur = c + step0.min(step1);
            step0 = step1;
            step1 = cur;
        }

        step0.min(step1)
    }
}
