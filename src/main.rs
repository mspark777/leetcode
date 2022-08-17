struct Solution {}
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);

        while (fast != 1) && (slow != fast) {
            slow = Self::get_next(slow);
            fast = Self::get_next(fast);
            fast = Self::get_next(fast);
        }

        fast == 1
    }

    #[inline]
    fn get_next(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 0 {
            let d = n % 10;
            result += d * d;
            n /= 10;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 19 }, Input { n: 2 }];

    for input in inputs {
        let n = input.n;
        let result = Solution::is_happy(n);
        println!("{:?}", result);
    }
}
