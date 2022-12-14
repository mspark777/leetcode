struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut robbed = 0;
        let mut no_robbed = 0;

        for n in nums.iter().cloned() {
            let temp = no_robbed;
            no_robbed = no_robbed.max(robbed);
            robbed = n + temp;
        }

        return robbed.max(no_robbed);
    }
}

fn main() {
    let inputs = [vec![1, 2, 3, 1], vec![2, 7, 9, 3, 1]];

    for nums in inputs {
        let result = Solution::rob(nums);
        println!("{result}");
    }
}
