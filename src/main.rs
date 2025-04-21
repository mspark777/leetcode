struct Solution {}

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut cur = 0;

        for &diff in &differences {
            cur += diff;
            x = x.min(cur);
            y = y.max(cur);

            if (y - x) > (upper - lower) {
                return 0;
            }
        }

        return (upper - lower) - (y - x) + 1;
    }
}

struct Input {
    differences: Vec<i32>,
    lower: i32,
    upper: i32,
}

fn main() {
    let inputs = vec![
        Input {
            differences: vec![1, -3, 4],
            lower: 1,
            upper: 6,
        },
        Input {
            differences: vec![3, -4, 5, 1, -2],
            lower: -4,
            upper: 5,
        },
        Input {
            differences: vec![4, -7, 2],
            lower: 3,
            upper: 6,
        },
    ];

    for input in inputs {
        let result = Solution::number_of_arrays(input.differences, input.lower, input.upper);
        println!("{result:?}");
    }
}
