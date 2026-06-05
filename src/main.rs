use std::collections::HashMap;

struct Codec {
    encode_map: HashMap<String, String>,
    docode_map: HashMap<String, String>,
    base: String,
}

impl Codec {
    fn new() -> Self {
        Self {
            encode_map: HashMap::new(),
            docode_map: HashMap::new(),
            base: "http://tinyurl.com/".to_string(),
        }
    }

    fn encode(&mut self, long_url: String) -> String {
        if let Some(short) = self.encode_map.get(&long_url) {
            short.clone()
        } else {
            let short = format!("{}{}", self.base, self.encode_map.len() + 1);
            self.encode_map.insert(long_url.clone(), short.clone());
            self.docode_map.insert(short.clone(), long_url);
            short
        }
    }

    fn decode(&self, short_url: String) -> String {
        self.docode_map[&short_url].clone()
    }
}

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut seen_set = HashSet::<i32>::new();
        let mut result_set = HashSet::<i32>::new();
        for num in nums.into_iter() {
            let n = num - k;
            if seen_set.contains(&n) {
                result_set.insert(n);
            }

            let n = num + k;
            if seen_set.contains(&n) {
                result_set.insert(num);
            }

            seen_set.insert(num);
        }
        result_set.len() as i32
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [Input {
        nums: [3, 1, 4, 1, 5].to_vec(),
        k: 2,
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_pairs(input.nums, input.k);
        println!("{:?}", result);
    }
}
