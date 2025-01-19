struct Solution {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    height: i32,
    row: usize,
    col: usize,
}

impl Cell {
    fn new(height: i32, row: usize, col: usize) -> Self {
        return Self { height, row, col };
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return if self.height < other.height {
            std::cmp::Ordering::Less
        } else if self.height > other.height {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        };
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let num_of_rows = height_map.len();
        let num_of_cols = height_map[0].len();
        let mut visited = vec![vec![false; num_of_cols]; num_of_rows];
        let mut queue = std::collections::BinaryHeap::<std::cmp::Reverse<Cell>>::with_capacity(
            num_of_rows * 2 + num_of_cols * 2,
        );

        for i in 0..num_of_rows {
            queue.push(std::cmp::Reverse(Cell::new(height_map[i][0], i, 0)));
            queue.push(std::cmp::Reverse(Cell::new(
                height_map[i][num_of_cols - 1],
                i,
                num_of_cols - 1,
            )));

            visited[i][0] = true;
            visited[i][num_of_cols - 1] = true;
        }

        for i in 0..num_of_cols {
            queue.push(std::cmp::Reverse(Cell::new(height_map[0][i], 0, i)));
            queue.push(std::cmp::Reverse(Cell::new(
                height_map[num_of_rows - 1][i],
                num_of_rows - 1,
                i,
            )));

            visited[0][i] = true;
            visited[num_of_rows - 1][i] = true;
        }

        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut result = 0;
        while let Some(std::cmp::Reverse(current_cell)) = queue.pop() {
            for (dr, dc) in dirs.iter().cloned() {
                let maybe_neighbor_row = Self::next(current_cell.row, dr, num_of_rows);
                let maybe_neighbor_col = Self::next(current_cell.col, dc, num_of_cols);
                if let (Some(neighbor_row), Some(neighbor_col)) =
                    (maybe_neighbor_row, maybe_neighbor_col)
                {
                    if visited[neighbor_row][neighbor_col] {
                        continue;
                    }

                    let neighbor_height = height_map[neighbor_row][neighbor_col];
                    if neighbor_height < current_cell.height {
                        result += current_cell.height - neighbor_height;
                    }

                    queue.push(std::cmp::Reverse(Cell::new(
                        neighbor_height.max(current_cell.height),
                        neighbor_row,
                        neighbor_col,
                    )));

                    visited[neighbor_row][neighbor_col] = true;
                }
            }
        }

        return result;
    }

    fn next(cur: usize, d: i32, end: usize) -> Option<usize> {
        let cur = cur as i64;
        let d = d as i64;
        let end = end as i64;
        let next = cur - d;
        return if next < 0 {
            None
        } else if next >= end {
            None
        } else {
            Some(next as usize)
        };
    }
}

struct Input {
    height_map: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            height_map: vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1],
            ],
        },
        Input {
            height_map: vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::trap_rain_water(input.height_map);
        println!("{result:?}");
    }
}
