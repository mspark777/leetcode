struct Solution {}

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut num_to_index = std::collections::HashMap::<i32, usize>::with_capacity(arr.len());
        for (i, n) in arr.iter().cloned().enumerate() {
            num_to_index.insert(n, i);
        }

        let mut result = i32::MAX;
        let num_rows = mat.len();
        let num_cols = mat[0].len();

        for row in 0..num_rows {
            let mut last_element_idx = i32::MIN;
            for col in 0..num_cols {
                let cell = mat[row][col];
                let &idx = num_to_index.get(&cell).unwrap();
                last_element_idx = last_element_idx.max(idx as i32);
            }

            result = result.min(last_element_idx);
        }

        for col in 0..num_cols {
            let mut last_element_idx = i32::MIN;
            for row in 0..num_rows {
                let cell = mat[row][col];
                let &idx = num_to_index.get(&cell).unwrap();
                last_element_idx = last_element_idx.max(idx as i32);
            }

            result = result.min(last_element_idx);
        }

        return result;
    }
}

struct Input {
    arr: Vec<i32>,
    mat: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![1, 3, 4, 2],
            mat: vec![vec![1, 4], vec![2, 3]],
        },
        Input {
            arr: vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            mat: vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]],
        },
    ];

    for input in inputs {
        let result = Solution::first_complete_index(input.arr, input.mat);
        println!("{result:?}");
    }
}
