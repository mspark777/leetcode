struct Solution {}
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as u64;
        let mut left = 1u64;
        let mut right = num / 2;
        while left <= right {
            let mid = (left + right) / 2;
            let square = mid * mid;
            if num < square {
                right = mid - 1;
            } else if num > square {
                left = mid + 1;
            } else {
                return true;
            }
        }

        return num == 1;
    }
}

fn main() {
    let inputs = [16, 14, 1, 808201];

    for num in inputs {
        let result = Solution::is_perfect_square(num);
        println!("{result}");
    }
}
