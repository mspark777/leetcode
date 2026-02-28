struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut stones = n;
        let mut removing = 10;

        while stones >= 0 {
            stones -= removing;
            removing -= 1;
        }

        (removing & 1) == 0
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 12 }, Input { n: 1 }];

    for input in inputs {
        let result = Solution::can_alice_win(input.n);
        println!("{:?}", result);
    }
}
