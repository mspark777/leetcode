struct Solution {}

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned_set = std::collections::HashSet::from_iter(banned.into_iter());
        return Self::solve(banned_set, n, max_sum);
    }

    fn solve(banned_set: std::collections::HashSet<i32>, n: i32, mut max_sum: i32) -> i32 {
        let mut result = 0;

        for num in 1..=n {
            if banned_set.contains(&num) {
                continue;
            }

            let diff = max_sum - num;
            if diff < 0 {
                return result;
            }

            max_sum -= num;
            result += 1;
        }

        return result;
    }
}

struct Input {
    banned: Vec<i32>,
    n: i32,
    max_sum: i32,
}

fn main() {
    let inputs = vec![
        Input {
            banned: vec![1, 6, 5],
            n: 5,
            max_sum: 6,
        },
        Input {
            banned: vec![1, 2, 3, 4, 5, 6, 7],
            n: 8,
            max_sum: 1,
        },
        Input {
            banned: vec![11],
            n: 7,
            max_sum: 50,
        },
    ];

    for input in inputs {
        let result = Solution::max_count(input.banned, input.n, input.max_sum);
        println!("{result}");
    }
}
