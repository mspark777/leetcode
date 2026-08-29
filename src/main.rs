struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh: i128 = 0;
        let mut rotten: i128 = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().copied().enumerate() {
                let orange: i128 = 1 << (y * 11 + x);
                match cell {
                    1 => fresh |= orange,
                    2 => rotten |= orange,
                    _ => {}
                }
            }
        }

        let mut minutes = 0;
        loop {
            fresh &= !rotten;
            rotten = fresh & (rotten >> 1 | rotten << 1 | rotten << 11 | rotten >> 11);
            if rotten == 0 {
                return if fresh != 0 { -1 } else { minutes };
            }
            minutes += 1
        }
    }
}

struct Input {
    start_value: i32,
    target: i32,
}

fn main() {
    let inputs = [Input {
        start_value: 2,
        target: 3,
    }];

    for input in inputs {
        let result = Solution::broken_calc(input.start_value, input.target);
        println!("{:?}", result);
    }
}
