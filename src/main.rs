use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_count = mat.len();
        let col_count = mat[0].len();

        let mut queues = HashMap::<i32, Vec<i32>>::with_capacity(row_count.max(col_count));

        for i in 0..row_count {
            for j in 0..col_count {
                let row = i as i32;
                let col = j as i32;
                let key = row - col;
                let value = mat[i][j];
                if let Some(queue) = queues.get_mut(&key) {
                    queue.push(value);
                } else {
                    queues.insert(key, vec![value]);
                }
            }
        }

        for queue in queues.values_mut() {
            queue.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut result = vec![vec![0; col_count]; row_count];
        for i in 0..row_count {
            let row = &mut result[i];
            for j in 0..col_count {
                let r = i as i32;
                let c = j as i32;
                let key = r - c;
                let queue = queues.get_mut(&key).unwrap();
                row[j] = queue.pop().unwrap();
            }
        }

        result
    }
}

struct Input {
    mat: Vec<Vec<i32>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            mat: vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]],
        },
        Input {
            mat: vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50],
            ],
        },
    ];

    for input in inputs.iter() {
        let mat = input.mat.clone();
        let result = Solution::diagonal_sort(mat);
        println!("{result:?}");
    }
}
