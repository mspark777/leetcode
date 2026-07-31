struct Solution;

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut target = target.abs();
        let mut k = 0;
        while target > 0 {
            k += 1;
            target -= k;
        }

        match target & 1 {
            0 => k,
            _ => k + 1 + (k & 1),
        }
    }
}

struct Input {
    target: i32,
}

fn main() {
    let inputs = [Input { target: 2 }, Input { target: 3 }];

    for input in inputs.into_iter() {
        let result = Solution::reach_number(input.target);
        println!("{:?}", result);
    }
}
