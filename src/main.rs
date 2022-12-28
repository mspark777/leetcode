use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut queue = BinaryHeap::<i32>::with_capacity(piles.len());
        let mut result = 0;

        for &pile in piles.iter() {
            result += pile;
            queue.push(pile);
        }

        for _ in 0..k {
            let mut value = queue.pop().unwrap();
            let remove = value / 2;
            result -= remove;
            value -= remove;

            queue.push(value);
        }

        return result;
    }
}

struct Input {
    piles: Vec<i32>,
    k: i32,
}
fn main() {
    let inputs = [
        Input {
            piles: vec![5, 4, 9],
            k: 2,
        },
        Input {
            piles: vec![4, 3, 6, 7],
            k: 3,
        },
    ];

    for Input { piles, k } in inputs {
        let result = Solution::min_stone_sum(piles, k);
        println!("{result}");
    }
}
