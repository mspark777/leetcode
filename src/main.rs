struct Solution;

impl Solution {
    pub fn num_rabbits(mut answers: Vec<i32>) -> i32 {
        answers.sort_unstable();

        answers
            .chunk_by(|a, b| a == b)
            .map(|group| {
                let group_size = group[0] + 1;
                (group[0] + group.len() as i32) / group_size * group_size
            })
            .sum()
    }
}

struct Input {
    answers: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        answers: vec![1, 1, 2],
    }];

    for input in inputs.into_iter() {
        let result = Solution::num_rabbits(input.answers);
        println!("{:?}", result);
    }
}
