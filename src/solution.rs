pub struct Solution {}

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let size = card_points.len() - k;
        let mut min = 0;
        for i in 0..size {
            min += card_points[i];
        }
        let mut sum = min;
        let mut cur = min;

        for i in 0..k {
            let p = card_points[i + size];
            sum += p;
            cur += p - card_points[i];
            min = min.min(cur);
        }

        sum - min
    }
}
