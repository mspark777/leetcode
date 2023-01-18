struct Solution {}
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut curmax = 0;
        let mut curmin = 0;
        let mut sum = 0;
        let mut maxsum = nums[0];
        let mut minsum = nums[0];

        for &num in nums.iter() {
            curmax = curmax.max(0) + num;
            curmin = curmin.min(0) + num;
            maxsum = maxsum.max(curmax);
            minsum = minsum.min(curmin);
            sum += num;
        }

        if sum == minsum {
            return maxsum;
        }

        return maxsum.max(sum - minsum);
    }
}

fn main() {
    let inputs = [vec![1, -2, 3, -2], vec![5, -3, 5], vec![-3, -2, -3]];

    for nums in inputs {
        let result = Solution::max_subarray_sum_circular(nums);
        println!("{result}");
    }
}
