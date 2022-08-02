struct Solution {}
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mlen = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[mlen - 1][mlen - 1];

        while left < right {
            let mid = left + ((right - left) / 2);
            let mut count = 0;

            for i in 0..mlen {
                for j in (0..mlen).rev() {
                    if matrix[i][j] <= mid {
                        count += j + 1;
                        break;
                    }
                }
            }

            if count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
    k: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            matrix: vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]],
            k: 8,
        },
        Input {
            matrix: vec![vec![-5]],
            k: 1,
        },
        Input {
            matrix: vec![vec![-5, -4], vec![-5, -4]],
            k: 2,
        },
        Input {
            matrix: vec![vec![1, 2], vec![1, 3]],
            k: 1,
        },
    ];

    for input in inputs {
        let matrix = input.matrix;
        let k = input.k;
        let result = Solution::kth_smallest(matrix, k);
        println!("{:?}", result);
    }
}
