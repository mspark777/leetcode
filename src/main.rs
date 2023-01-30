struct Solution {}
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 1 {
            return 0;
        } else if n < 3 {
            return 1;
        }

        return Self::recursive(n, 0, 1, 1);
    }

    fn recursive(n: i32, t0: i32, t1: i32, t2: i32) -> i32 {
        if n < 3 {
            return t2;
        }

        return Self::recursive(n - 1, t1, t2, t0 + t1 + t2);
    }
}

fn main() {
    let inputs = [4, 25];

    for n in inputs {
        let result = Solution::tribonacci(n);
        println!("{result}");
    }
}
