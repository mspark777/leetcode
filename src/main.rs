struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let winner = (1..n).fold(0usize, |i, j| match grid[i][j] {
            1 => i,
            _ => j,
        });

        winner as i32
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            grid: [[0, 1], [0, 0]].map(|r| r.to_vec()).to_vec(),
        },
        Input {
            grid: [[0, 0, 1], [1, 0, 1], [0, 0, 0]]
                .map(|r| r.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_champion(input.grid);
        println!("{:?}", result);
    }
}
