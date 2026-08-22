struct RLEIterator {
    encoding: Vec<i32>,
    index: usize,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self { encoding, index: 0 }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n = n;
        while self.index < self.encoding.len() {
            if n <= self.encoding[self.index] {
                self.encoding[self.index] -= n;
                return self.encoding[self.index + 1];
            } else {
                n -= self.encoding[self.index];
                self.index += 2;
            }
        }
        -1
    }
}

struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::iter::once;

        let mut ans = HashSet::<i32>::new();
        let mut cur = HashSet::<i32>::from([0]);

        for x in arr {
            cur = cur.into_iter().map(|y| x | y).chain(once(x)).collect();
            ans.extend(&cur);
        }

        ans.len() as i32
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input { arr: [0].to_vec() }];

    for input in inputs {
        let result = Solution::subarray_bitwise_o_rs(input.arr);
        println!("{:?}", result);
    }
}
