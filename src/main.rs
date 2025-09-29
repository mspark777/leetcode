struct Solution {}

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for row in 0..n {
            let mut seen = std::collections::HashSet::<i32>::with_capacity(n);
            for col in 0..n {
                if seen.contains(&matrix[row][col]) {
                    return false;
                }
                seen.insert(matrix[row][col]);
            }
        }

        for col in 0..n {
            let mut seen = std::collections::HashSet::<i32>::with_capacity(n);
            for row in 0..n {
                if seen.contains(&matrix[row][col]) {
                    return false;
                }
                seen.insert(matrix[row][col]);
            }
        }

        true
    }
}

struct Input {
    title: String,
}

fn main() {
    let inputs = [
        Input {
            title: "capiTalIze tHe titLe".to_string(),
        },
        Input {
            title: "First leTTeR of EACH Word".to_string(),
        },
        Input {
            title: "i lOve leetcode".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::capitalize_title(input.title);
        println!("{:?}", result);
    }
}
