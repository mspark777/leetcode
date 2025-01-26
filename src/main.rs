struct Solution {}

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counts = std::collections::HashMap::<i32, i32>::new();
        for c in deck.iter().cloned() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut gcd = *counts.get(&deck[0]).unwrap();
        counts.remove(&deck[0]);

        for count in counts.values().cloned() {
            gcd = Self::gcd(gcd, count);
        }

        return gcd >= 2;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        return if a == 0 { b } else { Self::gcd(b % a, a) };
    }
}

struct Input {
    deck: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            deck: vec![1, 2, 3, 4, 4, 3, 2, 1],
        },
        Input {
            deck: vec![1, 1, 1, 2, 2, 2, 3, 3],
        },
    ];

    for input in inputs {
        let result = Solution::has_groups_size_x(input.deck);
        println!("{result:?}");
    }
}
