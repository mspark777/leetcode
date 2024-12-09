struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_list = vec![0; nums.len()];

        for i in 1..nums.len() {
            let curr = nums[i] & 1;
            let prev = nums[i - 1] & 1;
            prefix_list[i] = prefix_list[i - 1];
            if curr == prev {
                prefix_list[i] += 1;
            }
        }

        let mut result = vec![false; queries.len()];
        for (i, query) in queries.iter().enumerate() {
            let start = query[0] as usize;
            let end = query[1] as usize;

            result[i] = prefix_list[start] == prefix_list[end];
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 4, 1, 2, 6],
            queries: vec![vec![0, 4]],
        },
        Input {
            nums: vec![4, 3, 1, 6],
            queries: vec![vec![0, 2], vec![2, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::is_array_special(input.nums, input.queries);
        println!("{result:?}");
    }
}
