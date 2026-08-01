struct Solution;

impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        for (i, n) in a.into_iter().enumerate() {
            let i = i as i32;
            if !(-1..=1).contains(&(n - i)) {
                return false;
            }
        }

        true
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: vec![1, 0, 2],
    }];

    for input in inputs.into_iter() {
        let result = Solution::is_ideal_permutation(input.nums);
        println!("{:?}", result);
    }
}
