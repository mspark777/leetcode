struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let a = grid[i][j];
                let b = grid[i][j + 1];
                let c = grid[i + 1][j];
                let d = grid[i + 1][j + 1];

                let ok = ((a == b) && (b == c))
                    || ((a == b) && (b == d) && (c != d))
                    || ((a == c) && (c == d) && (b != d))
                    || ((d == b) && (b == c) && (a != d));

                if ok {
                    return true;
                }
            }
        }

        false
    }
}

struct Input {
    grid: Vec<Vec<char>>,
}

fn main() {
    let inputs = [
        Input {
            grid: [['B', 'W', 'B'], ['B', 'W', 'W'], ['B', 'W', 'B']]
                .map(|a| a.to_vec())
                .to_vec(),
        },
        Input {
            grid: [['B', 'W', 'B'], ['W', 'B', 'W'], ['B', 'W', 'B']]
                .map(|a| a.to_vec())
                .to_vec(),
        },
        Input {
            grid: [['B', 'W', 'B'], ['B', 'W', 'W'], ['B', 'W', 'W']]
                .map(|a| a.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::can_make_square(input.grid);
        println!("{:?}", result);
    }
}
