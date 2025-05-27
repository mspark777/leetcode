struct Solution {}

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let k = n / m;
        return (n * (n + 1) / 2) - (k * (k + 1) * m);
    }
}

struct Input {
    n: i32,
    m: i32,
}

fn main() {
    let inputs = vec![
        Input { n: 10, m: 3 },
        Input { n: 5, m: 6 },
        Input { n: 5, m: 1 },
    ];

    for input in inputs {
        let result = Solution::difference_of_sums(input.n, input.m);
        println!("{:?}", result);
    }
}
