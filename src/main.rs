struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut chars = s1.chars();
        let s10 = chars.next().unwrap();
        let s11 = chars.next().unwrap();
        let s12 = chars.next().unwrap();
        let s13 = chars.next().unwrap();

        let mut chars = s2.chars();
        let s20 = chars.next().unwrap();
        let s21 = chars.next().unwrap();
        let s22 = chars.next().unwrap();
        let s23 = chars.next().unwrap();

        Self::check(s10, s12, s20, s22) && Self::check(s11, s13, s21, s23)
    }

    fn check(a: char, b: char, c: char, d: char) -> bool {
        (a == c && b == d) || (a == d && b == c)
    }
}

struct Input {
    s1: String,
    s2: String,
}

fn main() {
    let inputs = [
        Input {
            s1: "abcd".to_string(),
            s2: "cdab".to_string(),
        },
        Input {
            s1: "abcd".to_string(),
            s2: "dacb".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::can_be_equal(input.s1, input.s2);
        println!("{:?}", result);
    }
}
