pub struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut seen = vec![vec![false; col_count]; row_count];

        let dr = [1, -1, 0, 0];
        let dc = [0, 0, 1, -1];

        let mut result = 0;
        let mut stack = Vec::<(i32, i32)>::with_capacity(row_count);
        for r in 0..row_count {
            for c in 0..col_count {
                if (grid[r][c] == 0) || seen[r][c] {
                    continue;
                }

                let mut shape = 0;
                stack.push((r as i32, c as i32));
                seen[r][c] = true;
                while !stack.is_empty() {
                    let (row, col) = stack.pop().unwrap();
                    shape += 1;
                    for i in 0..4 {
                        let nr = row + dr[i];
                        let nc = col + dc[i];
                        if Self::out_range(nr, row_count) {
                            continue;
                        } else if Self::out_range(nc, col_count) {
                            continue;
                        }

                        let nr = nr as usize;
                        let nc = nc as usize;
                        if grid[nr][nc] == 0 {
                            continue;
                        } else if seen[nr][nc] {
                            continue;
                        }

                        stack.push((nr as i32, nc as i32));
                        seen[nr][nc] = true;
                    }
                }
                result = result.max(shape);
            }
        }
        result
    }

    #[inline]
    fn out_range(n: i32, end: usize) -> bool {
        (n < 0) || (n >= end as i32)
    }
}
