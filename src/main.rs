struct Solution {}

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let positive_count = nums.len() - Self::upper_bound(&nums);
        let negative_count = Self::lower_bound(&nums);

        return positive_count.max(negative_count) as i32;
    }

    fn lower_bound(nums: &Vec<i32>) -> usize {
        let mut start = 0usize;
        let mut end = nums.len() - 1;
        let mut idx = nums.len();

        while start <= end {
            let mid = (start + end) / 2;

            if nums[mid] < 0 {
                start = mid + 1;
            } else {
                if mid > 0 {
                    end = mid - 1;
                    idx = mid;
                } else {
                    idx = 0;
                    break;
                }
            }
        }

        return idx;
    }

    fn upper_bound(nums: &Vec<i32>) -> usize {
        let mut start = 0usize;
        let mut end = nums.len() - 1;
        let mut idx = nums.len();

        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid] <= 0 {
                start = mid + 1;
            } else {
                if mid > 0 {
                    end = mid - 1;
                    idx = mid;
                } else {
                    idx = 0;
                    break;
                }
            }
        }

        return idx;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![-2, -1, -1, 1, 2, 3],
        },
        Input {
            nums: vec![-3, -2, -1, 0, 0, 1, 2],
        },
        Input {
            nums: vec![5, 20, 66, 1314],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_count(input.nums);
        println!("{result:?}");
    }
}
