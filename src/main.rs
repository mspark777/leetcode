struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::with_capacity(image.len());
        for row in image.iter() {
            let row_len = row.len();
            let mut new_col = row.clone();
            let end = (row_len + 1) / 2;
            for i in 0..end {
                let j = row_len - (i + 1);
                new_col[i] = row[j] ^ 1;
                new_col[j] = row[i] ^ 1;
            }

            result.push(new_col);
        }

        return result;
    }
}

struct Input {
    image: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            image: vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]],
        },
        Input {
            image: vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::flip_and_invert_image(input.image);
        println!("{result:?}");
    }
}
