struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        let k = k as usize;
        let mut pq = BinaryHeap::with_capacity(k);
        for p in points {
            let d = p[0] * p[0] + p[1] * p[1];
            pq.push((d, vec![p[0], p[1]]));
            if pq.len() > k {
                pq.pop();
            }
        }

        pq.into_iter().map(|(_, p)| p).collect()
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
