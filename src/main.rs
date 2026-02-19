struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        let k = k ^ n;
        let cnt = k.count_ones();

        let k = k & n;
        match cnt == k.count_ones() {
            true => cnt as i32,
            _ => -1,
        }
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [
        Input { n: 13, k: 4 },
        Input { n: 21, k: 21 },
        Input { n: 14, k: 13 },
    ];

    for input in inputs {
        let result = Solution::min_changes(input.n, input.k);
        println!("{:?}", result);
    }
}
