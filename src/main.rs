struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        for n in &nums {
            total_sum += n;
        }

        let mut left_sum = 0;
        for (i, &n) in nums.iter().enumerate() {
            let right_sum = total_sum - (left_sum + n);
            if right_sum == left_sum {
                return i as i32;
            }

            left_sum += n;
        }

        return -1;
    }
}

fn main() {
    let inputs = vec![vec![1, 7, 3, 6, 5, 6], vec![1, 2, 3], vec![2, 1, -1]];

    for input in inputs {
        let result = Solution::pivot_index(input);
        println!("{result}");
    }
}
