struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diagonal = 0;
        let mut result = 0;

        for rect in dimensions {
            let length = rect[0];
            let width = rect[1];
            let diagonal = length * length + width * width;
            let area = length * width;

            if diagonal > max_diagonal {
                max_diagonal = diagonal;
                result = area;
            } else if diagonal == max_diagonal {
                result = result.max(area);
            }
        }

        result
    }
}

struct Input {
    dimensions: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            dimensions: [[9, 3], [8, 6]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            dimensions: [[3, 4], [4, 3]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::area_of_max_diagonal(input.dimensions);
        println!("{:?}", result);
    }
}
