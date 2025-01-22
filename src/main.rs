struct Solution {}

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = is_water.len();
        let columns = is_water[0].len();
        let inf = rows * columns;
        let mut result = vec![vec![inf; columns]; rows];

        for (r, row) in is_water.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    result[r][c] = 0;
                }
            }
        }

        for r in 0..rows {
            for c in 0..columns {
                let mut min_neighbor_distance = inf;

                let may_above_row = Self::valid_cell(r, -1, rows);
                if let Some(above_row) = may_above_row {
                    min_neighbor_distance = min_neighbor_distance.min(result[above_row][c]);
                }

                let may_left_column = Self::valid_cell(c, -1, columns);
                if let Some(left_column) = may_left_column {
                    min_neighbor_distance = min_neighbor_distance.min(result[r][left_column]);
                }

                result[r][c] = result[r][c].min(min_neighbor_distance + 1);
            }
        }

        for r in (0..rows).rev() {
            for c in (0..columns).rev() {
                let mut min_neighbor_distance = inf;

                let may_below_row = Self::valid_cell(r, 1, rows);
                if let Some(below_row) = may_below_row {
                    min_neighbor_distance = min_neighbor_distance.min(result[below_row][c]);
                }

                let may_right_column = Self::valid_cell(c, 1, columns);
                if let Some(right_column) = may_right_column {
                    min_neighbor_distance = min_neighbor_distance.min(result[r][right_column]);
                }

                result[r][c] = result[r][c].min(min_neighbor_distance + 1);
            }
        }

        return result
            .iter()
            .map(|r| r.iter().map(|&c| c as i32).collect())
            .collect();
    }

    fn valid_cell(r: usize, d: i32, max: usize) -> Option<usize> {
        let r = r as i32;
        let max = max as i32;
        let nr = r + d;
        return if nr < 0 {
            None
        } else if nr >= max {
            None
        } else {
            Some(nr as usize)
        };
    }
}

struct Input {
    is_water: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            is_water: vec![vec![0, 1], vec![0, 0]],
        },
        Input {
            is_water: vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]],
        },
    ];

    for input in inputs {
        let result = Solution::highest_peak(input.is_water);
        println!("{result:?}");
    }
}
