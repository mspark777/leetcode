use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let freqs = {
            let mut freqs = HashMap::<i32, i32>::new();
            for n in arr.iter() {
                if let Some(freq) = freqs.get_mut(n) {
                    *freq += 1;
                } else {
                    freqs.insert(*n, 1);
                }
            }
            freqs
        };

        let pqueue = {
            let mut q: Vec<i32> = freqs.values().map(|f| *f).collect();
            q.sort_unstable_by(|a, b| b.cmp(a));
            q
        };

        let mut deleted = 0;
        let mut result = 0;
        let half = (arr.len() / 2) as i32;
        for freq in pqueue {
            deleted += freq;
            result += 1;

            if deleted >= half {
                return result;
            }
        }

        -1
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            arr: vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7],
        },
        Input {
            arr: vec![7, 7, 7, 7, 7, 7],
        },
        Input {
            arr: vec![9, 77, 63, 22, 92, 9, 14, 54, 8, 38, 18, 19, 38, 68, 58, 19],
        },
    ];

    for input in inputs {
        let arr = input.arr;
        let result = Solution::min_set_size(arr);
        println!("{:?}", result);
    }
}
