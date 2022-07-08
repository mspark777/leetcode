pub struct Solution {}

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        const MAX_COST: i32 = 1000001;
        let mut prev_memo = Self::create_memo(target + 1, n, MAX_COST);

        for color in 1..=n {
            let color_index = color as usize;
            if houses[0] == color {
                prev_memo[1][color_index - 1] = 0;
            } else if houses[0] == 0 {
                prev_memo[1][color_index - 1] = cost[0][color_index - 1];
            }
        }

        for house in 1..m {
            let mut memo = Self::create_memo(target + 1, n, MAX_COST);

            for neighborhoods in 1..=target.min(house + 1) {
                let house = house as usize;
                let neighborhoods = neighborhoods as usize;
                for color in 1..=n {
                    if houses[house] != 0 && (color != houses[house]) {
                        continue;
                    }

                    let color = color as usize;
                    let mut curr_cost = MAX_COST;
                    for prev_color in 1..=n {
                        let prev_color = prev_color as usize;
                        if prev_color != color {
                            curr_cost = curr_cost.min(prev_memo[neighborhoods - 1][prev_color - 1]);
                        } else {
                            curr_cost = curr_cost.min(prev_memo[neighborhoods][color - 1]);
                        }
                    }

                    let cost_to_paint = if houses[house] != 0 {
                        0
                    } else {
                        cost[house][color - 1]
                    };
                    memo[neighborhoods][color - 1] = curr_cost + cost_to_paint;
                }
            }

            prev_memo = memo;
        }

        let mut min_cost = MAX_COST;
        for color in 1..=n {
            min_cost = min_cost.min(prev_memo[target as usize][(color - 1) as usize]);
        }

        if min_cost == MAX_COST {
            -1
        } else {
            min_cost
        }
    }

    fn create_memo(row: i32, col: i32, val: i32) -> Vec<Vec<i32>> {
        let row = row as usize;
        let col = col as usize;

        vec![vec![val; col]; row]
    }
}
