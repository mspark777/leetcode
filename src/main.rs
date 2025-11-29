struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut recevied_list = vec![false; n];
        let mut who = 0usize;
        let mut turn = 1usize;

        while !recevied_list[who] {
            recevied_list[who] = true;
            who = (who + (turn * k)) % n;
            turn += 1;
        }

        let mut result = Vec::<i32>::with_capacity(n + 1 - turn);
        for (i, received) in recevied_list.iter().copied().enumerate() {
            if received {
                continue;
            }
            let who = (i + 1) as i32;
            result.push(who);
        }

        result
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { n: 5, k: 2 }, Input { n: 4, k: 4 }];

    for input in inputs {
        let result = Solution::circular_game_losers(input.n, input.k);
        println!("{:?}", result);
    }
}
