mod utils;

use utils::Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = arr.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left as i32;
    }
}

fn main() {
    let inputs = [vec![0, 1, 0], vec![0, 2, 1, 0], vec![0, 10, 5, 2]];

    for arr in inputs {
        let result = Solution::peak_index_in_mountain_array(arr);
        println!("{result}");
    }
}
