struct Solution {}

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let money = money - children;
        if money < 0 {
            return -1;
        } else if ((money / 7) == children) && ((money % 7) == 0) {
            return children;
        } else if ((money / 7) == (children - 1)) && ((money % 7) == 3) {
            return children - 2;
        }

        (children - 1).min(money / 7)
    }
}

struct Input {
    money: i32,
    children: i32,
}

fn main() {
    let inputs = [
        Input {
            money: 20,
            children: 3,
        },
        Input {
            money: 16,
            children: 2,
        },
    ];

    for input in inputs {
        let result = Solution::dist_money(input.money, input.children);
        println!("{:?}", result);
    }
}
