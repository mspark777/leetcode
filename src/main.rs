struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; n + 1];

        for i in 1..=n {
            result[i] = Self::solve(i, &mut result);
        }

        return result;
    }

    fn solve(n: usize, memo: &mut Vec<i32>) -> i32 {
        if n <= 1 {
            return n as i32;
        } else if memo[n] != 0 {
            return memo[n];
        }

        memo[n] = Self::solve(n / 2, memo);
        if (n & 1) == 1 {
            memo[n] += 1;
        }

        return memo[n];
    }
}

fn main() {
    let inputs = [2, 5];

    for n in inputs {
        let result = Solution::count_bits(n);
        println!("{result:?}");
    }
}
