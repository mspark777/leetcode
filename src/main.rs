struct Solution {}
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut total_load = 0;
        let mut max_load = 0;

        for &weight in weights.iter() {
            total_load += weight;
            max_load = max_load.max(weight);
        }

        let mut left = max_load;
        let mut right = total_load;

        while left < right {
            let middle = (left + right) / 2;
            if Self::feasible(&weights, middle, days) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        return left;
    }

    fn feasible(weights: &Vec<i32>, capacity: i32, days: i32) -> bool {
        let mut days_needed = 1;
        let mut current_load = 0;

        for &weight in weights.iter() {
            current_load += weight;
            if current_load > capacity {
                days_needed += 1;
                current_load = weight;
            }

            if days_needed > days {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    weights: Vec<i32>,
    days: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            weights: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            days: 5,
        },
        Input {
            weights: vec![3, 2, 2, 4, 1, 4],
            days: 3,
        },
        Input {
            weights: vec![1, 2, 3, 1, 1],
            days: 4,
        },
    ];

    for Input { weights, days } in inputs {
        let result = Solution::ship_within_days(weights, days);
        println!("{result}");
    }
}
