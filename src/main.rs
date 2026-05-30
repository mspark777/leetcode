use rand::{Rng, rngs::ThreadRng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            rng: ThreadRng::default(),
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let r = self.rng.r#gen::<f64>().sqrt() * self.radius;
        let theta = self.rng.r#gen::<f64>() * 2.0 * std::f64::consts::PI;

        let x = self.x_center + r * theta.cos();
        let y = self.y_center + r * theta.sin();
        vec![x, y]
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [4, 14, 2].to_vec(),
        },
        Input {
            nums: [4, 14, 4].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::total_hamming_distance(input.nums);
        println!("{:?}", result);
    }
}
