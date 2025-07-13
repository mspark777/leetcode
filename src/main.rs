struct Solution {}

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;

        players.sort_unstable();
        trainers.sort_unstable();

        let mut result = 0;
        let mut i = 0usize;
        let mut j = 0usize;
        let m = players.len();
        let n = trainers.len();

        while (i < m) && (j < n) {
            while (j < n) && (players[i] > trainers[j]) {
                j += 1;
            }

            if j < n {
                result += 1;
            }

            i += 1;
            j += 1;
        }

        result
    }
}

struct Input {
    players: Vec<i32>,
    trainers: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            players: [4, 7, 9].to_vec(),
            trainers: [8, 2, 5, 8].to_vec(),
        },
        Input {
            players: [1, 1, 1].to_vec(),
            trainers: [10].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::match_players_and_trainers(input.players, input.trainers);
        println!("{:?}", result);
    }
}
