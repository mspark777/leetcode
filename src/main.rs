mod utils;

use utils::Solution;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let rows = pizza.len();
        let cols = pizza[0].len();
        let mut apples = Self::matrix(rows + 1, cols + 1);
        let mut f = Self::matrix(rows, cols);

        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                apples[row][col] = Self::btoi(pizza[row].as_bytes()[col] == b'A')
                    + apples[row + 1][col]
                    + apples[row][col + 1]
                    - apples[row + 1][col + 1];
                f[row][col] = Self::btoi(apples[row][col] > 0);
            }
        }

        const MOD: i32 = 1000000007;
        for _remain in 1..k {
            let mut g = Self::matrix(rows, cols);
            for row in 0..rows {
                for col in 0..cols {
                    for next_row in (row + 1)..rows {
                        if (apples[row][col] - apples[next_row][col]) > 0 {
                            g[row][col] += f[next_row][col];
                            g[row][col] %= MOD;
                        }
                    }

                    for next_col in (col + 1)..cols {
                        if (apples[row][col] - apples[row][next_col]) > 0 {
                            g[row][col] += f[row][next_col];
                            g[row][col] %= MOD;
                        }
                    }
                }
            }
            f = g;
        }
        return f[0][0];
    }

    fn matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
        return vec![vec![0; cols]; rows];
    }

    fn btoi(b: bool) -> i32 {
        return if b { 1 } else { 0 };
    }
}

fn main() {
    let inputs = [
        (vec!["A..", "AAA", "..."], 3),
        (vec!["A..", "AA.", "..."], 3),
        (vec!["A..", "A..", "..."], 1),
    ];

    for (pizza, k) in inputs {
        let pizza = pizza.iter().map(|s| s.to_string()).collect();
        let result = Solution::ways(pizza, k);
        println!("{result}");
    }
}
