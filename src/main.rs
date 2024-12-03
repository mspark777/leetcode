struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        return rec1[2].min(rec2[2]) > rec1[0].max(rec2[0])
            && rec1[3].min(rec2[3]) > rec1[1].max(rec2[1]);
    }
}

struct Input {
    rec1: Vec<i32>,
    rec2: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            rec1: vec![0, 0, 2, 2],
            rec2: vec![1, 1, 3, 3],
        },
        Input {
            rec1: vec![0, 0, 1, 1],
            rec2: vec![1, 0, 2, 1],
        },
        Input {
            rec1: vec![0, 0, 1, 1],
            rec2: vec![2, 2, 3, 3],
        },
    ];

    for input in inputs {
        let result = Solution::is_rectangle_overlap(input.rec1, input.rec2);
        println!("{result}");
    }
}
