struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut freq = [0; 61];
        let mut res = 0;

        for tim in time {
            let mut t = (tim % 60) as usize;
            res += freq[60 - t];
            if t == 0 {
                t = 60;
            }
            freq[t] += 1;
        }

        res
    }
}

struct Input {
    start_value: i32,
    target: i32,
}

fn main() {
    let inputs = [Input {
        start_value: 2,
        target: 3,
    }];

    for input in inputs {
        let result = Solution::broken_calc(input.start_value, input.target);
        println!("{:?}", result);
    }
}
