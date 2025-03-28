struct Solution {}

#[derive(Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    value: i32,
}

impl Cell {
    fn new(row: usize, col: usize, value: i32) -> Self {
        return Self { row, col, value };
    }
}

struct Query {
    idx: usize,
    value: i32,
}

impl Query {
    fn new(idx: usize, value: i32) -> Self {
        return Self { idx, value };
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        return Self {
            parent: vec![n; n],
            size: vec![1; n],
            n,
        };
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] >= self.n {
            return node;
        }

        self.parent[node] = self.find(self.parent[node]);
        return self.parent[node];
    }

    fn union(&mut self, node_a: usize, node_b: usize) -> bool {
        let root_a = self.find(node_a);
        let root_b = self.find(node_b);
        if root_a == root_b {
            return false;
        }

        if self.size[root_a] > self.size[root_b] {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        } else {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        }

        return true;
    }

    fn get_size(&mut self, node: usize) -> i32 {
        let parent = self.find(node);
        return self.size[parent] as i32;
    }
}

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let total_cells = row_count * col_count;

        let mut sorted_queries = queries
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, q)| Query::new(idx, q))
            .collect::<Vec<_>>();

        sorted_queries.sort_unstable_by_key(|q| q.value);

        let mut sorted_cells = Vec::<Cell>::with_capacity(total_cells);
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().cloned().enumerate() {
                sorted_cells.push(Cell::new(i, j, cell));
            }
        }

        sorted_cells.sort_unstable_by_key(|c| c.value);

        let mut uf = UnionFind::new(total_cells);
        let mut result = vec![0; queries.len()];
        let mut cell_idx = 0usize;

        let row_directions = [0, 0, 1, -1];
        let col_directions = [1, -1, 0, 0];

        for query in sorted_queries.iter() {
            while (cell_idx < total_cells) && (sorted_cells[cell_idx].value < query.value) {
                let cell = sorted_cells[cell_idx];
                let row = cell.row;
                let col = cell.col;
                let cell_id = row * col_count + col;

                for dir in 0..4usize {
                    let may_new_row = Self::next(row, row_directions[dir], row_count);
                    let may_new_col = Self::next(col, col_directions[dir], col_count);
                    if let (Some(new_row), Some(new_col)) = (may_new_row, may_new_col) {
                        if grid[new_row][new_col] < query.value {
                            uf.union(cell_id, new_row * col_count + new_col);
                        }
                    }
                }

                cell_idx += 1;
            }

            result[query.idx] = if query.value > grid[0][0] {
                uf.get_size(0)
            } else {
                0
            };
        }

        return result;
    }

    fn next(cur: usize, dir: i32, max: usize) -> Option<usize> {
        if dir >= 0 {
            let idx = cur + (dir as usize);
            return if idx < max { Some(idx) } else { None };
        }

        let dir = (-dir) as usize;
        if cur < dir {
            return None;
        }

        return Some(cur - dir);
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
    queries: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            queries: vec![5, 6, 2],
        },
        Input {
            grid: vec![vec![5, 2, 1], vec![1, 1, 2]],
            queries: vec![3],
        },
    ];

    for input in inputs {
        let result = Solution::max_points(input.grid, input.queries);
        println!("{result:?}");
    }
}
