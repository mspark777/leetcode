struct Solution {}

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let n = n as u32;
        let mut result = vec![0, 0];
        let mut idx = 1usize;
        for shift in (0..=31).rev() {
            result[idx] += ((n >> shift) & 1) as i32;

            idx ^= 1;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 50 }, Input { n: 2 }];

    for input in inputs {
        let result = Solution::even_odd_bit(input.n);
        println!("{:?}", result);
    }
}
