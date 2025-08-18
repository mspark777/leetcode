struct Solution {}

impl Solution {
    const EPS: f64 = 1e-6;

    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums: Vec<f64> = cards.iter().map(|&n| n as f64).collect();
        Self::dfs(&nums)
    }

    fn dfs(nums: &Vec<f64>) -> bool {
        if nums.len() == 1 {
            return (nums[0] - 24f64).abs() < Self::EPS;
        }

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                let mut next = Vec::<f64>::with_capacity(nums.len());
                for (k, n) in nums.iter().cloned().enumerate() {
                    if (k != i) && (k != j) {
                        next.push(n);
                    }
                }

                for num in Self::compute(nums[i], nums[j]) {
                    next.push(num);
                    if Self::dfs(&next) {
                        return true;
                    }
                    next.pop();
                }
            }
        }
        false
    }

    fn compute(a: f64, b: f64) -> Vec<f64> {
        let mut list = Vec::<f64>::with_capacity(6);
        list.push(a + b);
        list.push(a - b);
        list.push(b - a);
        list.push(a * b);

        if a.abs() > Self::EPS {
            list.push(b / a);
        }

        if b.abs() > Self::EPS {
            list.push(a / b);
        }

        list
    }
}

struct Input {
    cards: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            cards: [4, 1, 8, 7].to_vec(),
        },
        Input {
            cards: [1, 2, 1, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::judge_point24(input.cards);
        println!("{:?}", result);
    }
}
