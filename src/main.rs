struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();

        let a = nums.first().unwrap() + k;
        let b = nums.last().unwrap() - k;

        let mut result = nums[n - 1] - nums[0];
        for i in 0..(n - 1) {
            result = result.min(b.max(nums[i] + k) - a.min(nums[i + 1] - k));
        }

        result
    }
}

struct TopVotedCandidate {
    ld: Vec<i32>,
    ts: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        use std::collections::HashMap;

        let mut ld = Vec::with_capacity(persons.len());
        let mut m = HashMap::new();
        let mut b = (0, 0, 0);
        for (p, t) in persons.iter().copied().zip(times.iter().copied()) {
            let k = m.get(&p).map(|x: &(i32, i32)| x.0).unwrap_or(0);
            m.insert(p, (k + 1, t));
            b = b.max((k + 1, t, p));
            ld.push(b.2);
        }
        Self {
            ld,
            ts: times.clone(),
        }
    }

    fn q(&self, tf: i32) -> i32 {
        self.ld[self.ts.binary_search(&tf).unwrap_or_else(|err| err - 1)]
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
