#[derive(Clone, Copy)]
struct Transaction {
    spend: i32,
    profit: i32,
}

struct Solution {}
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k <= 0 {
            return 0;
        }

        let k = k as usize;
        let mut transactions = vec![
            Transaction {
                spend: i32::max_value(),
                profit: 0
            };
            k + 1
        ];

        for price in prices {
            for i in 1..=k {
                let prev = transactions[i - 1];
                let mut cur = &mut transactions[i];

                cur.spend = cur.spend.min(price - prev.profit);
                cur.profit = cur.profit.max(price - cur.spend);
            }
        }

        transactions[k].profit
    }
}

struct Input {
    k: i32,
    prices: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            k: 2,
            prices: vec![2, 4, 1],
        },
        Input {
            k: 2,
            prices: vec![3, 2, 6, 5, 0, 3],
        },
    ];

    for Input { k, prices } in inputs.iter() {
        let result = Solution::max_profit(*k, prices.clone());
        println!("{result:?}");
    }
}
