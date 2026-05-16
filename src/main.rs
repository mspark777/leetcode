struct Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut result = 0;

        for (i, a) in points.iter().enumerate() {
            let mut distance_map = HashMap::<i32, i32>::new();
            for (j, b) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dx = a[0] - b[0];
                let dy = a[1] - b[1];
                let distance = dx * dx + dy * dy;
                distance_map
                    .entry(distance)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }

            for count in distance_map.values().copied() {
                result += count * (count - 1);
            }
        }

        result
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            points: [[0, 0], [1, 0], [2, 0]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            points: [[1, 1], [2, 2], [3, 3]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            points: [[1, 1]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::number_of_boomerangs(input.points);
        println!("{:?}", result);
    }
}
