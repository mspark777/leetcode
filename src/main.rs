struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        Self::best(needs, &special, &price, &mut HashMap::new())
    }

    fn best(
        needs: Vec<i32>,
        special: &[Vec<i32>],
        price: &[i32],
        mem: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if let Some(val) = mem.get(&needs) {
            return *val;
        }
        let mut pos = Vec::new();
        pos.push(
            needs
                .iter()
                .zip(price.iter())
                .map(|(n, p)| n * p)
                .sum::<i32>(),
        );
        for spec in special {
            if needs.iter().zip(spec.iter()).all(|(n, s)| s <= n) {
                let mut cur = needs.clone();
                for (i, (_n, s)) in needs.iter().zip(spec.iter()).enumerate() {
                    cur[i] -= s;
                }
                pos.push(spec.last().unwrap() + Self::best(cur, special, price, mem));
            }
        }
        let val = pos.into_iter().min().unwrap();
        mem.insert(needs, val);
        val
    }
}

struct Input {
    price: Vec<i32>,
    special: Vec<Vec<i32>>,
    needs: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        price: [2, 5].to_vec(),
        special: [[3, 0, 5], [1, 2, 10]].map(|v| v.to_vec()).to_vec(),
        needs: [3, 2].to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::shopping_offers(input.price, input.special, input.needs);
        println!("{:?}", result);
    }
}
