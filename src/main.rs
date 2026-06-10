struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().bytes().collect::<Vec<u8>>();

        let mut i = digits.len() as i32 - 2;
        while i >= 0 && digits[i as usize] >= digits[(i + 1) as usize] {
            i -= 1;
        }

        if i < 0 {
            return -1;
        }

        let mut j = digits.len() as i32 - 1;
        while digits[j as usize] <= digits[i as usize] {
            j -= 1;
        }

        digits.swap(i as usize, j as usize);

        digits[(i as usize + 1)..].reverse();

        let res = digits
            .iter()
            .fold(0i64, |acc, &d| acc * 10 + (d - b'0') as i64);

        if res > i32::MAX as i64 {
            -1
        } else {
            res as i32
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 12 }, Input { n: 21 }];

    for input in inputs.into_iter() {
        let result = Solution::next_greater_element(input.n);
        println!("{:?}", result);
    }
}
