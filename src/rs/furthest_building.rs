use std::collections::BinaryHeap;

#[allow(dead_code)]
pub struct FurthestBuilding {}

#[allow(dead_code)]
impl FurthestBuilding {
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
