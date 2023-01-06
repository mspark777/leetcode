struct Solution {}
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        let mut coins = coins;

        costs.sort_unstable();

        let mut result = 0;
        for &cost in costs.iter() {
            if coins >= cost {
                coins -= cost;
                result += 1;
            } else {
                break;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 3, 2, 4, 1], 7),
        (vec![10, 6, 8, 7, 7, 8], 5),
        (vec![1, 6, 3, 1, 2, 5], 20),
    ];

    for (costs, coins) in inputs {
        let result = Solution::max_ice_cream(costs, coins);
        println!("{result}");
    }
}
