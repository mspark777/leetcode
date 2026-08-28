struct Solution;

impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut r = String::new();
        while (a + b) > 0 {
            match (r.ends_with("aa"), r.ends_with("bb"), a > b) {
                (true, _, _) => {
                    r.push('b');
                    b -= 1
                }
                (_, true, _) | (_, _, true) => {
                    r.push('a');
                    a -= 1
                }
                _ => {
                    r.push('b');
                    b -= 1
                }
            }
        }
        r
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
