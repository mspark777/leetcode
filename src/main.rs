struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let an = a.len();
        let bn = b.len();
        let x = bn / an + 2;

        a.repeat(x)
            .find(&b)
            .map(|i| ((i + bn - 1) / an) + 1)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}

struct Input {
    a: String,
    b: String,
}

fn main() {
    let inputs = [Input {
        a: "abcd".to_string(),
        b: "cdabcdab".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::repeated_string_match(input.a, input.b);
        println!("{:?}", result);
    }
}
