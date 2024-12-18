struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = prices.clone();
        let mut stack = Vec::<usize>::with_capacity(prices.len());

        for (i, &price) in prices.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if prices[j] < price {
                    break;
                } else {
                    result[j] -= price;
                    stack.pop();
                }
            }

            stack.push(i as usize);
        }

        return result;
    }
}

struct Input {
    prices: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            prices: vec![8, 4, 6, 2, 3],
        },
        Input {
            prices: vec![1, 2, 3, 4, 5],
        },
        Input {
            prices: vec![10, 1, 1, 6],
        },
    ];

    for input in inputs {
        let result = Solution::final_prices(input.prices);
        println!("{result:?}");
    }
}
