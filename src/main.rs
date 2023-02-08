/*
leetcode
 */

struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut curend = 0usize;
        let mut curfar = 0usize;

        for i in 0..(nums.len() - 1) {
            let n = nums[i] as usize;
            curfar = curfar.max(i + n);

            if i == curend {
                result += 1;
                curend = curfar;
            }
        }

        return result;
    }
}

fn main() {
    let inputs: Vec<Vec<i32>> = vec![vec![2, 3, 1, 1, 4], vec![2, 3, 0, 1, 4]];

    for input in inputs {
        let result = Solution::jump(input);
        println!("{result}");
    }
}
