use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}
impl Solution {
    pub fn max_performance(n: i32, speeds: Vec<i32>, efficiencies: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut candidates: Vec<(i32, i32)> =
            efficiencies.into_iter().zip(speeds.into_iter()).collect();
        candidates.sort_unstable_by_key(|c| -c.0);

        let mut speed_sum = 0u64;
        let mut result = 0u64;
        let mut queue = BinaryHeap::<Reverse<u64>>::with_capacity(n);

        for (efficiency, speed) in candidates.into_iter() {
            let speed = speed as u64;
            queue.push(Reverse(speed));
            speed_sum += speed;
            if queue.len() > k {
                speed_sum -= queue.pop().unwrap().0;
            }

            let efficiency = efficiency as u64;
            result = result.max(speed_sum * efficiency);
        }

        (result % 1000000007) as i32
    }
}

struct Input {
    n: i32,
    k: i32,
    speed: Vec<i32>,
    efficiency: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            n: 6,
            k: 2,
            speed: vec![2, 10, 3, 1, 5, 8],
            efficiency: vec![5, 4, 3, 9, 7, 2],
        },
        Input {
            n: 6,
            k: 3,
            speed: vec![2, 10, 3, 1, 5, 8],
            efficiency: vec![5, 4, 3, 9, 7, 2],
        },
        Input {
            n: 6,
            k: 4,
            speed: vec![2, 10, 3, 1, 5, 8],
            efficiency: vec![5, 4, 3, 9, 7, 2],
        },
    ];

    for Input {
        n,
        k,
        speed,
        efficiency,
    } in inputs.into_iter()
    {
        let result = Solution::max_performance(n, speed, efficiency, k);
        println!("{result}");
    }
}
