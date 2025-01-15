struct Solution {}

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dr_list = [0, 1, 0, -1];
        let dc_list = [1, 0, -1, 0];
        let n = grid.len();
        let mut result = 0;

        for r in 0..n {
            for c in 0..n {
                let cell = grid[r][c];
                if cell <= 0 {
                    continue;
                }

                result += 2;
                for (&dr, &dc) in dr_list.iter().zip(dc_list.iter()) {
                    let maybe_nr = Self::next(r, dr, n);
                    let maybe_nc = Self::next(c, dc, n);
                    let mut nv = 0;
                    if let (Some(nr), Some(nc)) = (maybe_nr, maybe_nc) {
                        nv = grid[nr][nc];
                    }

                    result += 0.max(grid[r][c] - nv);
                }
            }
        }

        return result;
    }

    fn next(u: usize, d: i32, n: usize) -> Option<usize> {
        if d == 0 {
            return Some(u);
        } else if d > 0 {
            let nu = u + d as usize;
            if nu < n {
                return Some(nu);
            }

            return None;
        }

        let d = (d * -1) as usize;
        if u < d {
            return None;
        }

        return Some(u - d);
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![1, 2], vec![3, 4]],
        },
        Input {
            grid: vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        },
        Input {
            grid: vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::surface_area(input.grid);
        println!("{result:?}");
    }
}
