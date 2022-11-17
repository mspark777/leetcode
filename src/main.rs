struct Solution {}
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let overx = ax2.min(bx2) - ax1.max(bx1);
        let overy = ay2.min(by2) - ay1.max(by1);

        let areaa = (ay2 - ay1) * (ax2 - ax1);
        let areab = (by2 - by1) * (bx2 - bx1);
        let areac = if (overx > 0) && (overy > 0) {
            overx * overy
        } else {
            0
        };

        return areaa.abs() + areab.abs() - areac;
    }
}

fn main() {
    let inputs = [[-3, 0, 3, 4, 0, -1, 9, 2], [-2, -2, 2, 2, -2, -2, 2, 2]];

    for [ax1, ay1, ax2, ay2, bx1, by1, bx2, by2] in inputs {
        let result = Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        println!("{result}");
    }
}
