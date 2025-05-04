struct Solution {}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut nums = vec![0; 100];
        let mut result = 0;

        for domino in dominoes.iter() {
            let a = domino[0];
            let b = domino[1];
            let num = (if a < b { a * 10 + b } else { b * 10 + a }) as usize;

            result += nums[num];
            nums[num] += 1;
        }

        return result;
    }
}

struct Input {
    dominoes: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            dominoes: [[1, 2], [2, 1], [3, 4], [5, 6]]
                .to_vec()
                .iter()
                .map(|s| s.to_vec())
                .collect(),
        },
        Input {
            dominoes: [[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]
                .to_vec()
                .iter()
                .map(|s| s.to_vec())
                .collect(),
        },
    ];

    for input in inputs {
        let result = Solution::num_equiv_domino_pairs(input.dominoes);
        println!("{result:?}");
    }
}
