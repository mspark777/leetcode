pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let ratings_len = ratings.len();
        if ratings_len < 2 {
            return ratings_len as i32;
        }

        let mut candies = 0;
        let mut up = 0;
        let mut down = 0;
        let mut old_slope = 0;

        for i in 1..ratings_len {
            let cur = ratings[i];
            let prev = ratings[i - 1];
            let slope = if cur > prev {
                1
            } else if cur < prev {
                -1
            } else {
                0
            };

            if ((old_slope > 0) && (slope == 0)) || ((old_slope < 0) && (slope >= 0)) {
                candies += Self::count(up) + Self::count(down) + up.max(down);
                up = 0;
                down = 0;
            }

            if slope > 0 {
                up += 1;
            } else if slope < 0 {
                down += 1;
            } else {
                candies += 1;
            }

            old_slope = slope;
        }

        candies += Self::count(up) + Self::count(down) + up.max(down) + 1;
        candies
    }

    fn count(n: i32) -> i32 {
        let n1 = n + 1;
        (n * n1) / 2
    }
}
