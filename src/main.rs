struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        Self::check(&p1, &p2, &p3, &p4)
            || Self::check(&p1, &p3, &p2, &p4)
            || Self::check(&p1, &p2, &p4, &p3)
    }

    fn dist(p1: &[i32], p2: &[i32]) -> i32 {
        (p2[1] - p1[1]) * (p2[1] - p1[1]) + (p2[0] - p1[0]) * (p2[0] - p1[0])
    }

    fn check(p1: &[i32], p2: &[i32], p3: &[i32], p4: &[i32]) -> bool {
        Self::dist(p1, p2) > 0
            && Self::dist(p1, p3) > 0
            && Self::dist(p1, p2) == Self::dist(p2, p3)
            && Self::dist(p2, p3) == Self::dist(p3, p4)
            && Self::dist(p3, p4) == Self::dist(p4, p1)
            && Self::dist(p1, p3) == Self::dist(p2, p4)
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [5, 4, 0, 3, 1, 6, 2].to_vec(),
        },
        Input {
            nums: [0, 1, 2].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::array_nesting(input.nums);
        println!("{:?}", result);
    }
}
