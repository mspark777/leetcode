struct Solution {}
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let bn = (3 * n) - 2;

        let mut bpadded = vec![vec![0; bn]; bn];

        for r in 0..n {
            for c in 0..n {
                bpadded[r + n - 1][c + n - 1] = img2[r][c];
            }
        }

        let sn = (2 * n) - 1;
        let mut max_overlaps = 0;
        for xshift in 0..sn {
            for yshift in 0..sn {
                max_overlaps = max_overlaps.max(Self::convolute(&img1, &bpadded, xshift, yshift));
            }
        }

        return max_overlaps;
    }

    fn convolute(img: &Vec<Vec<i32>>, kernel: &Vec<Vec<i32>>, xshift: usize, yshift: usize) -> i32 {
        let n = img.len();
        let mut result = 0;

        for r in 0..n {
            for c in 0..n {
                result += img[r][c] * kernel[r + yshift][c + xshift];
            }
        }

        return result;
    }
}

struct Input {
    img1: Vec<Vec<i32>>,
    img2: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            img1: vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            img2: vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
        },
        Input {
            img1: vec![vec![1]],
            img2: vec![vec![1]],
        },
        Input {
            img1: vec![vec![0]],
            img2: vec![vec![0]],
        },
        Input {
            img1: vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]],
            img2: vec![vec![0, 1, 1], vec![0, 0, 0], vec![0, 0, 0]],
        },
    ];

    for Input { img1, img2 } in inputs {
        let result = Solution::largest_overlap(img1, img2);
        println!("{result}");
    }
}
