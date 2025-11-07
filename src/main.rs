struct Solution {}

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        const ENEMY: i32 = 0;

        let mut result = 0i32;
        let mut j = 0usize;
        let n = forts.len();
        for i in 0..n {
            if forts[i] != ENEMY {
                if forts[j] == -forts[i] {
                    result = result.max(((i - j) as i32) - 1)
                }

                j = i;
            }
        }

        result
    }
}

struct Input {
    forts: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            forts: [1, 0, 0, -1, 0, 0, 0, 0, 1].to_vec(),
        },
        Input {
            forts: [0, 0, 1, -1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::capture_forts(input.forts);
        println!("{:?}", result);
    }
}
