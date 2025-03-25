struct Solution {}

impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        return Self::check_cuts(rectangles.clone(), 0) || Self::check_cuts(rectangles.clone(), 1);
    }

    fn check_cuts(mut rectangles: Vec<Vec<i32>>, dim: usize) -> bool {
        let mut gap_count = 0;

        rectangles.sort_unstable_by_key(|r| r[dim]);
        let mut furthest_end = rectangles[0][dim + 2];

        for rect in rectangles.iter().skip(1) {
            if furthest_end <= rect[dim] {
                gap_count += 1;
            }

            furthest_end = furthest_end.max(rect[dim + 2]);
        }

        return gap_count >= 2;
    }
}

struct Input {
    n: i32,
    rectangles: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 5,
            rectangles: vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5],
            ],
        },
        Input {
            n: 4,
            rectangles: vec![
                vec![0, 0, 1, 1],
                vec![2, 0, 3, 4],
                vec![0, 2, 2, 3],
                vec![3, 0, 4, 3],
            ],
        },
        Input {
            n: 4,
            rectangles: vec![
                vec![0, 2, 2, 4],
                vec![1, 0, 3, 2],
                vec![2, 2, 3, 4],
                vec![3, 0, 4, 2],
                vec![3, 2, 4, 4],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::check_valid_cuts(input.n, input.rectangles);
        println!("{result:?}");
    }
}
