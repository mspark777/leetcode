struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize;
        for row in mat.iter() {
            for (i, cell) in row.iter().copied().enumerate() {
                let j = (i + row.len() + k) % row.len();
                if cell != row[j] {
                    return false;
                }
            }
        }

        true
    }
}

struct Input {
    mat: Vec<Vec<i32>>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            mat: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
                .map(|r| r.to_vec())
                .to_vec(),
            k: 4,
        },
        Input {
            mat: [[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]]
                .map(|r| r.to_vec())
                .to_vec(),
            k: 2,
        },
        Input {
            mat: [[2, 2], [2, 2]].map(|r| r.to_vec()).to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::are_similar(input.mat, input.k);
        println!("{:?}", result);
    }
}
