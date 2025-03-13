struct Solution {}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut result = 0usize;
        let mut difference_array = vec![0; n + 1];

        for i in 0..n {
            while sum + difference_array[i] < nums[i] {
                result += 1;

                if result > queries.len() {
                    return -1;
                }

                let left = queries[result - 1][0] as usize;
                let right = queries[result - 1][1] as usize;
                let val = queries[result - 1][2];

                if right >= i {
                    difference_array[i.max(left)] += val;
                    difference_array[right + 1] -= val;
                }
            }

            sum += difference_array[i];
        }

        return result as i32;
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 0, 2],
            queries: vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]],
        },
        Input {
            nums: vec![4, 3, 2, 1],
            queries: vec![vec![1, 3, 2], vec![0, 2, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::min_zero_array(input.nums, input.queries);
        println!("{result:?}");
    }
}
