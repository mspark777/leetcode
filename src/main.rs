use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut heap = BinaryHeap::<i32>::new();
        let mut bricks = bricks;
        let mut ladders = ladders;

        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];
            if diff <= 0 {
                continue;
            }

            heap.push(diff);
            bricks -= diff;
            if bricks < 0 {
                let max = heap.pop().unwrap();
                bricks += max;
                ladders -= 1;
            }

            if ladders < 0 {
                return (i as i32) - 1;
            }
        }

        (heights.len() as i32) - 1
    }
}

struct Input {
    heights: Vec<i32>,
    bricks: i32,
    leadders: i32,
}

fn main() {
    let inputs = [
        Input {
            heights: vec![4, 2, 7, 6, 9, 14, 12],
            bricks: 5,
            leadders: 1,
        },
        Input {
            heights: vec![4, 12, 2, 7, 3, 18, 20, 3, 19],
            bricks: 10,
            leadders: 2,
        },
        Input {
            heights: vec![14, 3, 19, 3],
            bricks: 17,
            leadders: 0,
        },
        Input {
            heights: vec![1, 5, 1, 2, 3, 4, 10000],
            bricks: 4,
            leadders: 1,
        },
    ];

    for input in inputs {
        let result = Solution::furthest_building(input.heights, input.bricks, input.leadders);
        println!("{result:?}");
    }
}
