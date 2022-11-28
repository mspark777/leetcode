use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lose_counts = HashMap::<i32, i32>::new();

        for m in matches.iter() {
            lose_counts.entry(m[0]).or_insert(0);
            *lose_counts.entry(m[1]).or_insert(0) += 1;
        }

        let mut winners = Vec::<i32>::new();
        let mut losers = Vec::<i32>::new();
        for (player, count) in lose_counts.iter() {
            let p = *player;
            let c = *count;
            if c < 1 {
                winners.push(p);
            } else if c == 1 {
                losers.push(p);
            }
        }

        winners.sort_unstable();
        losers.sort_unstable();

        return vec![winners, losers];
    }
}

fn main() {
    let inputs = [
        vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ],
        vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]],
    ];

    for matches in inputs {
        let result = Solution::find_winners(matches);
        println!("{result:?}");
    }
}
