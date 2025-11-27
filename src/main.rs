struct Solution {}

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut prev1 = [0, 0];
        let mut prev2 = [0, 0];
        let mut score1 = 0;
        let mut score2 = 0;
        for (p1, p2) in player1.iter().copied().zip(player2.iter().copied()) {
            let double1 = (prev1[0] >= 10) || (prev1[1] >= 10);
            if double1 {
                score1 += p1 * 2;
            } else {
                score1 += p1;
            }

            let double2 = (prev2[0] >= 10) || (prev2[1] >= 10);
            if double2 {
                score2 += p2 * 2;
            } else {
                score2 += p2;
            }

            prev1[0] = prev1[1];
            prev1[1] = p1;

            prev2[0] = prev2[1];
            prev2[1] = p2;
        }

        if score1 < score2 {
            2
        } else if score1 > score2 {
            1
        } else {
            0
        }
    }
}

struct Input {
    player1: Vec<i32>,
    player2: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            player1: [5, 10, 3, 2].to_vec(),
            player2: [6, 5, 7, 3].to_vec(),
        },
        Input {
            player1: [3, 5, 7, 6].to_vec(),
            player2: [8, 10, 10, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::is_winner(input.player1, input.player2);
        println!("{:?}", result);
    }
}
