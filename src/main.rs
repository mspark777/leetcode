struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[1]);

        let mut result = 1;
        let mut prev = 0usize;

        for (cur, point) in points.iter().skip(1).enumerate() {
            if point[0] > points[prev][1] {
                result += 1;
                prev = cur + 1;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]],
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
    ];

    for points in inputs {
        let result = Solution::find_min_arrow_shots(points);
        println!("{result}");
    }
}
