struct Solution {}
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|k| -k);
        for i in 0..=(nums.len() - 3) {
            let a = nums[i];
            let b = nums[i + 1] + nums[i + 2];
            if a < b {
                return a + b;
            }
        }

        return 0;
    }
}

fn main() {
    let inputs = [vec![2, 1, 2], vec![1, 2, 1]];

    for nums in inputs {
        let result = Solution::largest_perimeter(nums);
        println!("{result}");
    }
}
