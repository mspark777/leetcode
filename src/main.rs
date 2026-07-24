struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;

        let mut f = vec![vec![vec![0.0; n]; n]; k + 1];

        for row in &mut f[0] {
            for cell in row.iter_mut() {
                *cell = 1.0;
            }
        }

        let dirs: [i32; 9] = [-2, -1, 2, 1, -2, 1, 2, -1, -2];

        for h in 1..=k {
            for i in 0..n {
                for j in 0..n {
                    for p in 0..8 {
                        let x = (i as i32) + dirs[p];
                        let y = (j as i32) + dirs[p + 1];

                        if x >= 0 && x < (n as i32) && y >= 0 && y < (n as i32) {
                            let x = x as usize;
                            let y = y as usize;
                            f[h][i][j] += f[h - 1][x][y] / 8.0;
                        }
                    }
                }
            }
        }

        f[k][row as usize][column as usize]
    }
}

struct Input {
    n: i32,
    k: i32,
    row: i32,
    column: i32,
}

fn main() {
    let inputs = [Input {
        n: 3,
        k: 2,
        row: 0,
        column: 0,
    }];

    for input in inputs.into_iter() {
        let result = Solution::knight_probability(input.n, input.k, input.row, input.column);
        println!("{:?}", result);
    }
}
