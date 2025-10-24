struct Solution {}

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let flush_set = std::collections::HashSet::<char>::from_iter(suits.iter().cloned());
        if flush_set.len() == 1 {
            return "Flush".to_string();
        }

        let mut same_ranks = std::collections::HashMap::<i32, i32>::with_capacity(ranks.len());
        let mut same_count = 0;
        for rank in ranks.iter().cloned() {
            let count = same_ranks.entry(rank).and_modify(|e| *e += 1).or_insert(1);
            same_count = same_count.max(*count);
        }

        return match same_count {
            2 => "Pair",
            1 | 0 => "High Card",
            _ => "Three of a Kind",
        }
        .to_string();
    }
}

struct Input {
    ranks: Vec<i32>,
    suits: Vec<char>,
}

fn main() {
    let inputs = [
        Input {
            ranks: [13, 2, 3, 1, 9].to_vec(),
            suits: ['a', 'a', 'a', 'a', 'a'].to_vec(),
        },
        Input {
            ranks: [4, 4, 2, 4, 4].to_vec(),
            suits: ['d', 'a', 'a', 'b', 'c'].to_vec(),
        },
        Input {
            ranks: [10, 10, 2, 12, 9].to_vec(),
            suits: ['a', 'b', 'c', 'a', 'd'].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::best_hand(input.ranks, input.suits);
        println!("{:?}", result);
    }
}
