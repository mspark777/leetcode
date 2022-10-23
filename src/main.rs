struct Solution {}
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut temps = vec![0; nums.len()];
        for num in nums.into_iter() {
            let i = (num - 1) as usize;
            temps[i] += 1;
        }

        let mut missing = 0usize;
        let mut dup = 0usize;

        for (i, temp) in temps.into_iter().enumerate() {
            if temp <= 0 {
                missing = i;
            } else if temp > 1 {
                dup = i;
            }

            if (missing > 0) && (dup > 0) {
                break;
            }
        }

        return vec![(dup + 1) as i32, (missing + 1) as i32];
    }
}

fn main() {
    let inputs = [vec![1, 2, 2, 4], vec![1, 1]];

    for nums in inputs {
        let result = Solution::find_error_nums(nums);
        println!("{result:?}");
    }
}
