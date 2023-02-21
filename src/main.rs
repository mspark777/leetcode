struct Solution {}
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let middle = (left + right) / 2;
            if (middle & 1) == 1 {
                if nums[middle] != nums[middle + 1] {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            } else {
                if nums[middle] == nums[middle + 1] {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
        }

        return nums[left];
    }
}

fn main() {
    let inputs: Vec<Vec<i32>> = vec![
        vec![1, 1, 2, 3, 3, 4, 4, 8, 8],
        vec![3, 3, 7, 7, 10, 11, 11],
    ];

    for nums in inputs {
        let result = Solution::single_non_duplicate(nums);
        println!("{result}");
    }
}
