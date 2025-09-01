struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut result = -1;
        let mut smallest_distance = i32::MAX;

        for (i, point) in points.iter().enumerate() {
            let px = point[0];
            let py = point[1];
            if px == x || py == y {
                let distance = (x - px).abs() + (y - py).abs();
                if distance < smallest_distance {
                    result = i as i32;
                    smallest_distance = distance;
                }
            }
        }

        result
    }
}

struct Input {
    x: i32,
    y: i32,
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            x: 3,
            y: 4,
            points: [[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            x: 3,
            y: 4,
            points: [[3, 4]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            x: 3,
            y: 4,
            points: [[2, 3]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::nearest_valid_point(input.x, input.y, input.points);
        println!("{:?}", result);
    }
}
