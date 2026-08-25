struct Solution;

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let mut set: HashSet<(i32, i32)> = HashSet::new();
        let mut result = i32::MAX;

        for (i, p1) in points.iter().enumerate() {
            let x1 = p1[0];
            let y1 = p1[1];

            for p2 in points.iter().take(i) {
                let x2 = p2[0];
                let y2 = p2[1];
                if (x1 == x2) || (y1 == y2) {
                    continue;
                }

                let ar = (x1 - x2).abs() * (y1 - y2).abs();
                if (result > ar) && set.contains(&(x1, y2)) && set.contains(&(x2, y1)) {
                    result = ar;
                }
            }

            set.insert((x1, y1));
        }

        match result {
            i32::MAX => 0,
            _ => result,
        }
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        points: [[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]]
            .map(|v| v.to_vec())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::min_area_rect(input.points);
        println!("{:?}", result);
    }
}
