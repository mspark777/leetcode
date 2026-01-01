struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let all_cases = ((n + 2) * (n + 1)) / 2;

        if (3 * limit) < n {
            return 0;
        }

        if (3 * limit) == n {
            return 1;
        }

        let exclude_one = n - (limit + 1) + 2;
        let one_limit = match exclude_one > 1 {
            true => 3 * ((exclude_one * (exclude_one - 1)) / 2),
            _ => 0,
        };

        let exclude_two = n - 2 * (limit + 1) + 2;
        let two_limit = match exclude_two > 1 {
            true => 3 * ((exclude_two * (exclude_two - 1)) / 2),
            _ => 0,
        };

        let exclude_three = n - 3 * (limit + 1) + 2;
        let three_limit = match exclude_three > 1 {
            true => (exclude_three * (exclude_three - 1)) / 2,
            _ => 0,
        };

        all_cases - one_limit + two_limit - three_limit
    }
}

struct Input {
    n: i32,
    limit: i32,
}

fn main() {
    let inputs = [Input { n: 5, limit: 2 }, Input { n: 3, limit: 3 }];

    for input in inputs {
        let result = Solution::distribute_candies(input.n, input.limit);
        println!("{:?}", result);
    }
}
