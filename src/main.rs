struct Solution {}

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut accumulates = nums.clone();
        accumulates.sort_unstable();

        for i in 1..accumulates.len() {
            accumulates[i] += accumulates[i - 1];
        }

        let mut result = vec![0; queries.len()];
        for (i, query) in queries.iter().cloned().enumerate() {
            let mut left = 0usize;
            let mut right = nums.len();

            while left < right {
                let midx = (left + right) / 2;
                if accumulates[midx] <= query {
                    left = midx + 1;
                    result[i] = left as i32;
                } else {
                    right = midx;
                }
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [4, 5, 2, 1].to_vec(),
            queries: [3, 10, 21].to_vec(),
        },
        Input {
            nums: [2, 3, 4, 5].to_vec(),
            queries: [1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::answer_queries(input.nums, input.queries);
        println!("{:?}", result);
    }
}
