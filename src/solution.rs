pub struct Solution {}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort_unstable();

        let mut max_h = 0;
        for i in 1..horizontal_cuts.len() {
            max_h = max_h.max(horizontal_cuts[i] - horizontal_cuts[i - 1]);
        }

        let mut vertical_cuts = vertical_cuts;
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort_unstable();

        let mut max_w = 0;
        for i in 1..vertical_cuts.len() {
            max_w = max_w.max(vertical_cuts[i] - vertical_cuts[i - 1]);
        }

        let result = (max_h as i64) * (max_w as i64) % 1000000007;
        result as i32
    }
}
