struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let mut rounded = purchase_amount + 5;
        rounded -= rounded % 10;

        100 - rounded
    }
}

struct Input {
    purchase_amount: i32,
}

fn main() {
    let inputs = [
        Input { purchase_amount: 9 },
        Input {
            purchase_amount: 15,
        },
        Input {
            purchase_amount: 10,
        },
    ];

    for input in inputs {
        let result = Solution::account_balance_after_purchase(input.purchase_amount);
        println!("{:?}", result);
    }
}
