struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut rows = Vec::<Vec<i32>>::with_capacity(num_rows);
        for i in 0..num_rows {
            let mut row = vec![1; i + 1];
            let i = i as i32;
            let prev = i - 1;
            for j in 1..i {
                let j = j as usize;
                let prev = prev as usize;
                row[j] = rows[prev][j - 1] + rows[prev][j];
            }

            rows.push(row);
        }

        rows
    }
}

struct Input {
    num_rows: i32,
}

fn main() {
    let inputs = [Input { num_rows: 5 }, Input { num_rows: 1 }];

    for input in inputs {
        let result = Solution::generate(input.num_rows);
        println!("{:?}", result);
    }
}
