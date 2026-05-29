struct Solution;

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let n = heaters.len();
        let mut heaters = heaters;
        heaters.sort();

        let mut result = 0;

        for house in houses {
            let closest = match heaters.binary_search(&house) {
                Ok(i) => heaters[i],
                Err(i) => {
                    if i == 0 {
                        heaters[0]
                    } else if i == n {
                        heaters[n - 1]
                    } else {
                        let left = heaters[i - 1];
                        let right = heaters[i];
                        if (house - left).abs() <= (house - right).abs() {
                            left
                        } else {
                            right
                        }
                    }
                }
            };

            result = (house - closest).abs().max(result);
        }

        result
    }
}

struct Input {
    houses: Vec<i32>,
    heaters: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            houses: [1, 2, 3].to_vec(),
            heaters: [2].to_vec(),
        },
        Input {
            houses: [1, 2, 3, 4].to_vec(),
            heaters: [1, 4].to_vec(),
        },
        Input {
            houses: [1, 5].to_vec(),
            heaters: [2].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_radius(input.houses, input.heaters);
        println!("{:?}", result);
    }
}
