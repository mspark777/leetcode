struct Solution {}

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row_idx = 0usize;
        let mut count_ones = 0usize;

        for (i, row) in mat.iter().enumerate() {
            let count = row.iter().copied().filter(|&n| n == 1).count();
            if count > count_ones {
                row_idx = i;
                count_ones = count
            }
        }

        vec![row_idx as i32, count_ones as i32]
    }
}

struct Input {
    mat: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            mat: [[0, 1], [1, 0]].map(|a| a.to_vec()).to_vec(),
        },
        Input {
            mat: [[0, 0, 0], [0, 1, 1]].map(|a| a.to_vec()).to_vec(),
        },
        Input {
            mat: [[0, 0], [1, 1], [0, 0]].map(|a| a.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::row_and_maximum_ones(input.mat);
        println!("{:?}", result);
    }
}
