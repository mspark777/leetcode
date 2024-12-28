struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max_left_score = values[0];
        let mut result = 0;

        for (i, &score) in values.iter().skip(1).enumerate() {
            let j = (i + 1) as i32;
            result = result.max(max_left_score + score - j);
            max_left_score = max_left_score.max(score + j);
        }

        return result;
    }
}

struct Input {
    values: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            values: vec![8, 1, 5, 2, 6],
        },
        Input { values: vec![1, 2] },
    ];

    for input in inputs {
        let result = Solution::max_score_sightseeing_pair(input.values);
        println!("{result:?}");
    }
}
