struct Solution {}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        return Self::cal(n + 2) - 3 * Self::cal(n - limit + 1)
            + 3 * Self::cal(n - (limit + 1) * 2 + 2)
            - Self::cal(n - 3 * (limit + 1) + 2);
    }

    fn cal(x: i32) -> i64 {
        if x < 0 {
            return 0;
        }

        let x = x as i64;
        return x * (x - 1) / 2;
    }
}

struct Input {
    n: i32,
    limit: i32,
}

fn main() {
    let inputs = vec![Input { n: 5, limit: 2 }, Input { n: 3, limit: 3 }];

    for input in inputs {
        let result = Solution::distribute_candies(input.n, input.limit);
        println!("{:?}", result);
    }
}
