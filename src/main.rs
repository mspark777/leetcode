struct Solution {}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0.0f64;

        for i in points.iter() {
            for j in points.iter() {
                for k in points.iter() {
                    let area = (i[0] * j[1] + j[0] * k[1] + k[0] * i[1]
                        - j[0] * i[1]
                        - k[0] * j[1]
                        - i[0] * k[1])
                        .abs();
                    max_area = max_area.max(area as f64);
                }
            }
        }

        return max_area * 0.5;
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            points: vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]],
        },
        Input {
            points: vec![vec![1, 0], vec![0, 0], vec![0, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::largest_triangle_area(input.points);
        println!("{result}");
    }
}
