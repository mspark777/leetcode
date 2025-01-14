struct Solution {}

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a = alice_sizes.iter().sum::<i32>();
        let sum_b = bob_sizes.iter().sum::<i32>();
        let diff = (sum_a - sum_b) / 2;
        let alice_set = std::collections::HashSet::<i32>::from_iter(alice_sizes.iter().cloned());

        for bob in bob_sizes.iter().cloned() {
            let target = bob + diff;
            if alice_set.contains(&target) {
                return vec![bob + diff, bob];
            }
        }

        return bob_sizes;
    }
}

struct Input {
    alice_sizes: Vec<i32>,
    bob_sizes: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            alice_sizes: vec![1, 1],
            bob_sizes: vec![2, 2],
        },
        Input {
            alice_sizes: vec![1, 2],
            bob_sizes: vec![2, 3],
        },
        Input {
            alice_sizes: vec![2],
            bob_sizes: vec![1, 3],
        },
    ];

    for input in inputs {
        let result = Solution::fair_candy_swap(input.alice_sizes, input.bob_sizes);
        println!("{result:?}");
    }
}
