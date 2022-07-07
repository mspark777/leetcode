mod solution;

use solution::Solution;

struct Input {
    s1: String,
    s2: String,
    s3: String,
}

fn main() {
    let inputs = [
        Input {
            s1: String::from("aabcc"),
            s2: String::from("dbbca"),
            s3: String::from("aadbbcbcac"),
        },
        Input {
            s1: String::from("aabcc"),
            s2: String::from("dbbca"),
            s3: String::from("aadbbbaccc"),
        },
        Input {
            s1: String::from(""),
            s2: String::from(""),
            s3: String::from(""),
        },
    ];

    for input in inputs {
        let result = Solution::is_interleave(input.s1, input.s2, input.s3);
        println!("{result:?}");
    }
}
