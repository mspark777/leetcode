struct Solution;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let c = wall
            .iter()
            .fold(HashMap::new(), |mut acc, row| {
                row.iter()
                    .take(row.len() - 1)
                    .scan(0, |state, &x| {
                        *state += x;
                        Some(*state)
                    })
                    .for_each(|x| {
                        acc.entry(x).and_modify(|c| *c += 1).or_insert(1);
                    });
                acc
            })
            .values()
            .max()
            .copied()
            .unwrap_or(0);

        (wall.len() as i32) - c
    }
}

struct Input {
    wall: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        wall: vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ],
    }];

    for input in inputs.into_iter() {
        let result = Solution::least_bricks(input.wall);
        println!("{:?}", result);
    }
}
