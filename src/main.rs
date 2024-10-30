struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut result = image.clone();
        Self::solve(&mut result, sr as usize, sc as usize, color);
        return result;
    }

    fn solve(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, color: i32) {
        let target_color = image[sr][sc];
        if target_color != color {
            Self::dfs(image, sr, sc, target_color, color);
        }
    }

    fn dfs(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, target_color: i32, new_color: i32) {
        let mut stack = vec![(sr, sc)];
        let last_r = image.len() - 1;
        let last_c = image[0].len() - 1;

        while let Some((top_r, top_c)) = stack.pop() {
            let cell_color = image[top_r][top_c];
            if cell_color != target_color {
                continue;
            }

            image[top_r][top_c] = new_color;
            if top_r > 0 {
                stack.push((top_r - 1, top_c));
            }

            if top_r < last_r {
                stack.push((top_r + 1, top_c));
            }

            if top_c > 0 {
                stack.push((top_r, top_c - 1));
            }

            if top_c < last_c {
                stack.push((top_r, top_c + 1));
            }
        }
    }
}

struct Input {
    image: Vec<Vec<i32>>,
    sr: i32,
    sc: i32,
    color: i32,
}

fn main() {
    let inputs = vec![
        Input {
            image: vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]],
            sr: 1,
            sc: 1,
            color: 2,
        },
        Input {
            image: vec![vec![0, 0, 0], vec![0, 0, 0]],
            sr: 0,
            sc: 0,
            color: 0,
        },
    ];

    for input in inputs {
        let result = Solution::flood_fill(input.image, input.sr, input.sc, input.color);
        println!("{result:?}");
    }
}
