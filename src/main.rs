use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 {
            return 1;
        }

        let mut result = 2;

        for i in 0..points.len() {
            let mut slopes = HashMap::<u64, i32>::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }

                let pointi = &points[i];
                let pointj = &points[j];
                let mut x = pointj[0] - pointi[0];
                let mut y = pointj[1] - pointi[1];
                let gcd = Self::gcd(x.abs(), y.abs());
                if gcd != 0 {
                    x /= gcd;
                    y /= gcd;
                }

                let left = x as u32;
                let right = y as u32;
                let key = ((left as u64) << 32) | right as u64;
                *slopes.entry(key).or_insert(0) += 1;
            }

            for &value in slopes.values() {
                result = result.max(value + 1);
            }
        }

        return result;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }

        return Self::gcd(b, a % b);
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 1], vec![2, 2], vec![3, 3]],
        vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ],
    ];

    let it = -2;
    let temp = it as u32;
    println!("{temp}");

    for points in inputs {
        let result = Solution::max_points(points);
        println!("{result}");
    }
}
