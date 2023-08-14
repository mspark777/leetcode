mod utils;

use utils::Solution;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        let mut left = 0usize;
        let mut right = col_count - 1;
        let mut top = 0usize;
        let mut bottom = row_count - 1;
        let mut dir = Direction::RIGHT;
        let mut result = Vec::<i32>::with_capacity(row_count * col_count);

        while (left <= right) && (top <= bottom) {
            if dir == Direction::RIGHT {
                for col in left..=right {
                    result.push(matrix[top][col]);
                }
                top += 1;
                dir = Direction::DOWN
            } else if dir == Direction::DOWN {
                for row in top..=bottom {
                    result.push(matrix[row][right]);
                }

                if right < 1 {
                    break;
                }

                right -= 1;
                dir = Direction::LEFT;
            } else if dir == Direction::LEFT {
                for col in (left..=right).rev() {
                    result.push(matrix[bottom][col]);
                }

                if bottom < 1 {
                    break;
                }

                bottom -= 1;
                dir = Direction::UP;
            } else {
                for row in (top..=bottom).rev() {
                    result.push(matrix[row][left]);
                }

                left += 1;
                dir = Direction::RIGHT;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
        vec![vec![3], vec![2]],
    ];

    for matrix in inputs {
        let result = Solution::spiral_order(matrix);
        println!("{result:?}");
    }
}
