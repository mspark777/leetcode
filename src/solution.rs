pub struct Solution {}
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let start_row = start_row as usize;
        let start_column = start_column as usize;

        const MODULO: i32 = 1000000007;
        let mut dp = vec![vec![0; n]; m];
        dp[start_row][start_column] = 1;

        let mut count = 0;
        for _ in 0..max_move {
            let mut temp = vec![vec![0; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if i == (m - 1) {
                        count = (count + dp[i][j]) % MODULO;
                    }

                    if j == (n - 1) {
                        count = (count + dp[i][j]) % MODULO;
                    }

                    if i == 0 {
                        count = (count + dp[i][j]) % MODULO;
                    }

                    if j == 0 {
                        count = (count + dp[i][j]) % MODULO;
                    }

                    let mut ti = 0;
                    if i > 0 {
                        ti += dp[i - 1][j];
                    }

                    if i < (m - 1) {
                        ti += dp[i + 1][j];
                    }
                    ti %= MODULO;

                    let mut tj = 0;
                    if j > 0 {
                        tj += dp[i][j - 1];
                    }

                    if j < (n - 1) {
                        tj += dp[i][j + 1];
                    }
                    tj %= MODULO;

                    temp[i][j] = (ti + tj) % MODULO;
                }
            }
            dp = temp;
        }

        count
    }
}
