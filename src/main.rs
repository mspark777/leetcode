struct Solution;

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        let mut top = x;
        let mut bottom = x + k - 1;
        let left = y;
        let right = y + k - 1;

        while top < bottom {
            #[allow(clippy::needless_range_loop)]
            for c in left..=right {
                let t = grid[top][c];
                grid[top][c] = grid[bottom][c];
                grid[bottom][c] = t;
            }

            top += 1;
            bottom -= 1;
        }

        grid
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
    x: i32,
    y: i32,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            grid: [
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
            .map(|v| v.to_vec())
            .to_vec(),
            x: 1,
            y: 0,
            k: 3,
        },
        Input {
            grid: [[3, 4, 2, 3], [2, 3, 4, 2]].map(|v| v.to_vec()).to_vec(),
            x: 0,
            y: 2,
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::reverse_submatrix(input.grid, input.x, input.y, input.k);
        println!("{:?}", result);
    }
}
