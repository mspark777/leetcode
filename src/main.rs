struct Solution;

impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (n * n).min(max_weight / w)
    }
}

struct Input {
    n: i32,
    w: i32,
    max_weight: i32,
}

fn main() {
    let inputs = [
        Input {
            n: 2,
            w: 3,
            max_weight: 15,
        },
        Input {
            n: 3,
            w: 5,
            max_weight: 20,
        },
    ];

    for input in inputs {
        let result = Solution::max_containers(input.n, input.w, input.max_weight);
        println!("{:?}", result);
    }
}
