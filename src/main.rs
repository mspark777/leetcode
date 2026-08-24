struct Solution;

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![1];
        while result.len() < n {
            result = result
                .iter()
                .copied()
                .map(|m| m * 2 - 1)
                .chain(result.iter().copied().map(|m| m * 2))
                .filter(|&m| m <= (n as i32))
                .collect();
        }
        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 4 }];

    for input in inputs {
        let result = Solution::beautiful_array(input.n);
        println!("{:?}", result);
    }
}
