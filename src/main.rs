struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut i = 1i64;
        let mut count = 9i64;
        let mut start = 1i64;

        while n > (i * count) {
            n -= i * count;
            i += 1;
            count *= 10;
            start *= 10;
        }

        let nums = (start + ((n - 1) / i)).to_string();
        let nums = nums.as_bytes();
        let idx = ((n - 1) % i) as usize;
        let num = nums[idx];

        (num - b'0') as i32
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }, Input { n: 11 }, Input { n: 1000000000 }];

    for input in inputs.into_iter() {
        let result = Solution::find_nth_digit(input.n);
        println!("{:?}", result);
    }
}
