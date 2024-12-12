struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut queue = std::collections::binary_heap::BinaryHeap::from_iter(gifts.iter().cloned());

        for _ in 0..k {
            let top = queue.pop().unwrap();
            if top == 1 {
                return gifts.len() as i64;
            } else if top == 0 {
                return 0;
            } else {
                queue.push((top as f64).sqrt() as i32)
            }
        }

        let mut result = 0i64;
        for &n in queue.iter() {
            result += n as i64;
        }

        return result;
    }
}

struct Input {
    gifts: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            gifts: vec![25, 64, 9, 4, 100],
            k: 4,
        },
        Input {
            gifts: vec![1, 1, 1, 1],
            k: 4,
        },
    ];

    for input in inputs {
        let result = Solution::pick_gifts(input.gifts, input.k);
        println!("{result}");
    }
}
