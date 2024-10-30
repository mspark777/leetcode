struct Solution {}

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let lis_length = Self::get_longest_increasing_subsequence_length(&nums);

        let mut rev_nums = nums.clone();
        rev_nums.reverse();

        let lds_length = Self::get_longest_increasing_subsequence_length(&rev_nums);

        let mut result = i64::MAX;
        for (&lis, &lds) in lis_length.iter().zip(lds_length.iter().rev()) {
            if lis <= 1 || lds <= 1 {
                continue;
            }

            let v = Self::as_i64(nums.len()) - Self::as_i64(lis) - Self::as_i64(lds) + 1;
            result = result.min(v);
        }

        return result as i32;
    }

    fn as_i64(u: usize) -> i64 {
        return u as i64;
    }

    fn lower_bound(lis: &Vec<i32>, target: i32) -> usize {
        let mut left = 0usize;
        let mut right = lis.len();
        while left < right {
            let mid = (left + right) / 2;
            if lis[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }

    fn get_longest_increasing_subsequence_length(nums: &Vec<i32>) -> Vec<usize> {
        let mut lis_len = vec![1usize; nums.len()];
        let mut lis = vec![nums[0]];

        for (i, &num) in nums.iter().enumerate().skip(1) {
            let idx = Self::lower_bound(&lis, num);

            if idx == lis.len() {
                lis.push(num);
            } else {
                lis[idx] = num;
            }

            lis_len[i] = lis.len();
        }

        return lis_len;
    }
}

fn main() {
    let inputs = vec![vec![1, 3, 1], vec![2, 1, 1, 5, 6, 2, 3, 1]];

    for input in inputs {
        let result = Solution::minimum_mountain_removals(input);
        println!("{result:?}");
    }
}
