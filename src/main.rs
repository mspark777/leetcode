struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        (1..=n)
            .into_iter()
            .map(|n| format!("{:b}", n))
            .all(|n| s.find(n.as_str()).is_some())
    }
}

struct Input {
    s: String,
    n: i32,
}

fn main() {
    let inputs = [Input {
        s: "0110".to_string(),
        n: 3,
    }];

    for input in inputs {
        let result = Solution::query_string(input.s, input.n);
        println!("{:?}", result);
    }
}
