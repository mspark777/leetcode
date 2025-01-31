struct Solution {}

struct DisjointSet {
    parent_list: Vec<usize>,
    island_size_list: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let parent_list = (0..n).collect::<Vec<usize>>();
        let island_size_list = vec![1; n];

        return Self {
            parent_list,
            island_size_list,
        };
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent_list[node] == node {
            return node;
        }

        self.parent_list[node] = self.find(self.parent_list[node]);
        return self.parent_list[node];
    }

    fn union(&mut self, node_a: usize, node_b: usize) {
        let root_a = self.find(node_a);
        let root_b = self.find(node_b);

        if root_a == root_b {
            return;
        }

        if self.island_size_list[root_a] < self.island_size_list[root_b] {
            self.parent_list[root_a] = root_b;
            self.island_size_list[root_b] += self.island_size_list[root_a];
        } else {
            self.parent_list[root_b] = root_a;
            self.island_size_list[root_a] += self.island_size_list[root_b];
        }
    }
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid[0].len();
        let mut ds = DisjointSet::new(rows * columns);

        let row_directions = [-1, 1, 0, 0];
        let column_directions = [0, 0, 1, -1];

        for current_row in 0..rows {
            for current_column in 0..columns {
                if grid[current_row][current_column] == 0 {
                    continue;
                }

                let current_node = (columns * current_row) + current_column;
                for direction in 0..4usize {
                    let neighbor_row = (current_row as i32) + row_directions[direction];
                    let neighbor_column = (current_column as i32) + column_directions[direction];
                    if (neighbor_row < 0) || (neighbor_column < 0) {
                        continue;
                    }

                    let neighbor_row = neighbor_row as usize;
                    if neighbor_row >= rows {
                        continue;
                    }

                    let neighbor_column = neighbor_column as usize;
                    if neighbor_column >= columns {
                        continue;
                    }

                    if grid[neighbor_row][neighbor_column] == 1 {
                        let neighbor_node = (columns * neighbor_row) + neighbor_column;
                        ds.union(current_node, neighbor_node);
                    }
                }
            }
        }

        let mut max_island_size = 0usize;
        let mut has_zero = false;
        let mut unique_roots = std::collections::HashSet::<usize>::new();

        for current_row in 0..rows {
            for current_column in 0..columns {
                if grid[current_row][current_column] == 1 {
                    continue;
                }

                has_zero = true;
                let mut current_island_size = 1;

                for direction in 0..4usize {
                    let neighbor_row = (current_row as i32) + row_directions[direction];
                    let neighbor_column = (current_column as i32) + column_directions[direction];
                    if (neighbor_row < 0) || (neighbor_column < 0) {
                        continue;
                    }

                    let neighbor_row = neighbor_row as usize;
                    if neighbor_row >= rows {
                        continue;
                    }

                    let neighbor_column = neighbor_column as usize;
                    if neighbor_column >= columns {
                        continue;
                    }

                    if grid[neighbor_row][neighbor_column] == 1 {
                        let neighbor_node = (columns * neighbor_row) + neighbor_column;
                        let root = ds.find(neighbor_node);
                        unique_roots.insert(root);
                    }
                }

                for root in unique_roots.iter().cloned() {
                    current_island_size += ds.island_size_list[root];
                }

                max_island_size = max_island_size.max(current_island_size);
                unique_roots.clear();
            }
        }

        let result = if has_zero {
            max_island_size
        } else {
            rows * columns
        };

        return result as i32;
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![1, 0], vec![0, 1]],
        },
        Input {
            grid: vec![vec![1, 1], vec![1, 0]],
        },
        Input {
            grid: vec![vec![1, 1], vec![1, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::largest_island(input.grid);
        println!("{result:?}");
    }
}
