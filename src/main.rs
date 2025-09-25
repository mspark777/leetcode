struct Solution {}

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let &first = colors.first().unwrap();
        let &last = colors.last().unwrap();

        colors
            .iter()
            .cloned()
            .enumerate()
            .fold(0usize, |result, (i, color)| {
                let mut r = result;
                if color != first {
                    r = r.max(i);
                }

                if color != last {
                    r = r.max(n - i - 1);
                }

                r
            }) as i32
    }
}

struct Input {
    colors: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            colors: [1, 1, 1, 6, 1, 1, 1].to_vec(),
        },
        Input {
            colors: [1, 8, 3, 8, 3].to_vec(),
        },
        Input {
            colors: [0, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_distance(input.colors);
        println!("{:?}", result);
    }
}
