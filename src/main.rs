struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x0 = coordinates[0][0];
        let y0 = coordinates[0][1];
        let x1 = coordinates[1][0];
        let y1 = coordinates[1][1];
        let dx0 = x1 - x0;
        let dy0 = y1 - y0;

        for coordinate in coordinates.iter().skip(2) {
            let x = coordinate[0];
            let y = coordinate[1];
            let dx = x1 - x;
            let dy = y1 - y;

            if (dy0 * dx) != (dy * dx0) {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    coordinates: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            coordinates: [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            coordinates: [[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::check_straight_line(input.coordinates);
        println!("{:?}", result);
    }
}
