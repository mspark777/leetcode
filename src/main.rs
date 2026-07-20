struct Solution;

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut c = 0usize;
        let mut result = vec![0; n];

        for v in 1..(n - k) {
            result[c] = v as i32;
            c += 1;
        }

        for i in 0..=k {
            result[c] = match i & 1 {
                0 => (n + (i / 2) - k) as i32,
                _ => (n - (i / 2)) as i32,
            };
            c += 1;
        }

        result
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { n: 3, k: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::construct_array(input.n, input.k);
        println!("{:?}", result);
    }
}
