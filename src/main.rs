struct Solution {}

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let rows = rows as usize;
        let cols = cols as usize;
        let mut result = Vec::<Vec<i32>>::with_capacity(rows * cols);

        for r in 0..rows {
            for c in 0..cols {
                let r = r as i32;
                let c = c as i32;
                let distance = (r - r_center).abs() + (c - c_center).abs();
                result.push(vec![r, c, distance]);
            }
        }

        result.sort_unstable_by_key(|nums| nums[2]);
        for nums in result.iter_mut() {
            nums.pop();
        }

        return result;
    }
}

struct Input {
    rows: i32,
    cols: i32,
    r_center: i32,
    c_center: i32,
}

fn main() {
    let inputs = vec![
        Input {
            rows: 1,
            cols: 2,
            r_center: 0,
            c_center: 0,
        },
        Input {
            rows: 2,
            cols: 2,
            r_center: 0,
            c_center: 1,
        },
        Input {
            rows: 2,
            cols: 3,
            r_center: 1,
            c_center: 2,
        },
    ];

    for input in inputs {
        let result =
            Solution::all_cells_dist_order(input.rows, input.cols, input.r_center, input.c_center);
        println!("{result:?}");
    }
}
