struct Solution;

impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        let mut answer = false;
        let mut l = 0;
        let mut r = 0;
        let mut f = true;
        for (cur, target) in start.chars().zip(result.chars()) {
            match cur {
                'R' if l > 0 => f = false,
                'R' => r += 1,
                _ => (),
            };

            match target {
                'L' if r > 0 => f = false,
                'L' => l += 1,
                'R' if r == 0 => f = false,
                'R' => r -= 1,
                _ => (),
            };

            match cur {
                'L' if l == 0 => f = false,
                'L' => l -= 1,
                _ => (),
            };

            if f {
                answer = (l == 0) && (r == 0);
            } else {
                answer = false;
            }
        }

        answer
    }
}

struct Input {
    start: String,
    result: String,
}

fn main() {
    let inputs = [Input {
        start: "RXXLRXRXL".to_string(),
        result: "XRLXXRRLX".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::can_transform(input.start, input.result);
        println!("{:?}", result);
    }
}
