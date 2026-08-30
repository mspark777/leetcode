struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut v = [a, b, c];
        v.sort_unstable();

        let a = v[0];
        let b = v[1];
        let c = v[2];

        if (c - a - 2) == 0 {
            return vec![0, 0];
        } else if ((b - a) < 3) || ((c - b) < 3) {
            return vec![1, c - a - 2];
        }

        vec![2, c - a - 2]
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
