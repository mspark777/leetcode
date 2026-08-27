struct Solution;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let points = points
            .iter()
            .map(|v| v.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut res = 0;
        let mut m = std::collections::HashMap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let pt_i_0 = points[i][0];
                let pt_i_1 = points[i][1];
                let pt_j_0 = points[j][0];
                let pt_j_1 = points[j][1];
                let center = (((pt_i_0 + pt_j_0) as u64) << 16) + (pt_i_1 + pt_j_1) as u64;
                let v = vec![pt_i_0, pt_i_1, pt_j_0, pt_j_1];
                m.entry(center).or_insert_with(Vec::new).push(v);
            }
        }
        for (_center, points) in m {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let p1 = &points[i];
                    let p2 = &points[j];
                    if (p1[0] - p2[0]) * (p1[0] - p2[2]) + (p1[1] - p2[1]) * (p1[1] - p2[3]) == 0 {
                        let area = Self::d2(p1[0], p1[1], p2[0], p2[1])
                            * Self::d2(p1[0], p1[1], p2[2], p2[3]);
                        if res == 0 || res > area {
                            res = area;
                        }
                    }
                }
            }
        }
        (res as f64).sqrt()
    }

    fn d2(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
        (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        points: [[1, 2], [2, 1], [1, 0], [0, 1]]
            .map(|v| v.to_vec())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::min_area_free_rect(input.points);
        println!("{:?}", result);
    }
}
