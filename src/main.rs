struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let n = n as usize;
        let mines_set = mines
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .collect::<HashSet<(usize, usize)>>();

        let mut dp = vec![vec![n; n]; n];

        for (r, row) in dp.iter_mut().enumerate() {
            let mut count = 0;
            for (c, cell) in row.iter_mut().enumerate() {
                count = match mines_set.contains(&(r, c)) {
                    true => 0,
                    _ => count + 1,
                };
                *cell = count;
            }

            count = 0;
            for (c, cell) in row.iter_mut().enumerate().rev() {
                count = match mines_set.contains(&(r, c)) {
                    true => 0,
                    _ => count + 1,
                };
                *cell = count.min(*cell);
            }
        }

        for c in 0..n {
            let mut count = 0;
            for (r, row) in dp.iter_mut().enumerate() {
                count = match mines_set.contains(&(r, c)) {
                    true => 0,
                    _ => count + 1,
                };
                row[c] = row[c].min(count);
            }
            count = 0;
            for (r, row) in dp.iter_mut().enumerate().rev() {
                count = match mines_set.contains(&(r, c)) {
                    true => 0,
                    _ => count + 1,
                };
                row[c] = row[c].min(count);
            }
        }

        dp.into_iter().flatten().max().unwrap_or_default() as i32
    }
}

struct Input {
    n: i32,
    mines: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        n: 5,
        mines: [[4, 2]].map(|v| v.to_vec()).to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::order_of_largest_plus_sign(input.n, input.mines);
        println!("{:?}", result);
    }
}
