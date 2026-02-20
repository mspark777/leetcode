struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut counts = vec![[0usize; 11]; n + 1];

        for p in pick {
            let x = p[0] as usize;
            let y = p[1] as usize;

            counts[x][y] += 1;
        }

        let mut result = 0;
        for (i, count) in counts.into_iter().enumerate() {
            for cnt in count {
                if cnt > i {
                    result += 1;
                    break;
                }
            }
        }

        result
    }
}

struct Input {
    n: i32,
    pick: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 4,
            pick: [[0, 0], [1, 0], [1, 0], [2, 1], [2, 1], [2, 0]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            n: 5,
            pick: [[1, 1], [1, 2], [1, 3], [1, 4]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            n: 5,
            pick: [[1, 1], [2, 4], [2, 4], [2, 4]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::winning_player_count(input.n, input.pick);
        println!("{:?}", result);
    }
}
