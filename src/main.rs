struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last = nums.len() as i32 - 1;
        for i in (0..last).rev() {
            let cur = i + nums[i as usize];
            if cur >= last {
                last = i;
            }
        }

        return last < 1;
    }
}

fn main() {
    let inputs = [vec![2, 3, 1, 1, 4], vec![3, 2, 1, 0, 4]];

    for nums in inputs {
        let result = Solution::can_jump(nums);
        println!("{result}");
    }
}
