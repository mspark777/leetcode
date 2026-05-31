use rand::Rng;
use std::collections::BTreeMap;

struct Solution {
    total_sum: i32,
    mp: BTreeMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut mp = BTreeMap::new();
        let mut total_sum = 0;

        for rec in rects {
            let x1 = rec[0];
            let y1 = rec[1];
            let x2 = rec[2];
            let y2 = rec[3];
            total_sum += (x2 - x1 + 1) * (y2 - y1 + 1);
            mp.insert(total_sum, rec);
        }

        Self { total_sum, mp }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let area = rng.gen_range(0..=self.total_sum);
        let rec = self.mp.range(area..).next().unwrap().1;
        let x1 = rec[0];
        let y1 = rec[1];
        let x2 = rec[2];
        let y2 = rec[3];
        let x = rng.gen_range(x1..=x2);
        let y = rng.gen_range(y1..=y2);

        vec![x, y]
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 6 }, Input { n: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::magical_string(input.n);
        println!("{:?}", result);
    }
}
