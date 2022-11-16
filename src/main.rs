struct Solution {}
impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let m = (left + right) / 2;
            let res = guess(m);
            if res < 0 {
                right = m - 1;
            } else if res > 0 {
                left = m + 1;
            } else {
                return m;
            }
        }

        return -1;
    }
}

fn main() {
    let inputs = [(10, 6), (1, 1), (2, 1)];

    for (n, pick) in inputs {
        unsafe {
            let result = Solution::guessNumber(n);
            println!("{result}");
        }
    }
}
