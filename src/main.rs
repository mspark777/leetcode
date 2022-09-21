struct Solution {}
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut sum = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            if (num % 2) == 0 {
                sum += num;
            }
        }

        let mut result = vec![0; queries.len()];
        for (i, query) in queries.iter().enumerate() {
            let index = query[1] as usize;
            let mut num = nums[index];
            if (num % 2) == 0 {
                sum -= num;
            }

            let val = query[0];
            num += val;
            if (num % 2) == 0 {
                sum += num;
            }

            nums[index] = num;
            result[i] = sum;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3, 4],
            queries: vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
        },
        Input {
            nums: vec![1],
            queries: vec![vec![4, 0]],
        },
    ];

    for Input { nums, queries } in inputs.into_iter() {
        let result = Solution::sum_even_after_queries(nums, queries);
        println!("{result:?}");
    }
}
