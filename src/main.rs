struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut n0 = 1;
        let mut n1 = 1;

        for _ in 1..n {
            let sum = n0 + n1;
            n0 = n1;
            n1 = sum;
        }

        return n1;
    }
}

fn main() {
    let inputs = [2, 3];

    for n in inputs {
        let result = Solution::climb_stairs(n);
        println!("{result}");
    }
}
