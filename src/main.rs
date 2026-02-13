struct Solution;

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let n = n - 1;
        let rounds = k / n;
        let rem = k % n;

        match rounds & 1 {
            1 => n - rem,
            _ => rem,
        }
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [
        Input { n: 3, k: 5 },
        Input { n: 5, k: 6 },
        Input { n: 4, k: 2 },
    ];

    for input in inputs {
        let result = Solution::number_of_child(input.n, input.k);
        println!("{:?}", result);
    }
}
