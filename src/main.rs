struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        match original.len() == (m * n) {
            true => original.chunks(n).map(|chunk| chunk.to_vec()).collect(),
            _ => vec![],
        }
    }
}

struct Input {
    original: Vec<i32>,
    m: i32,
    n: i32,
}

fn main() {
    let inputs = [
        Input {
            original: [1, 2, 3, 4].to_vec(),
            m: 2,
            n: 2,
        },
        Input {
            original: [1, 2, 3].to_vec(),
            m: 1,
            n: 3,
        },
        Input {
            original: [1, 2].to_vec(),
            m: 1,
            n: 1,
        },
    ];

    for input in inputs {
        let result = Solution::construct2_d_array(input.original, input.m, input.n);
        println!("{:?}", result);
    }
}
