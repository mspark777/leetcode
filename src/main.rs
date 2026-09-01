struct Solution;

use std::collections::HashSet;

impl Solution {
    fn r(hs: &mut HashSet<String>, cur: Vec<char>, rem: Vec<char>) {
        if rem.len() == 0 {
            return;
        }

        for i in 0..rem.len() {
            let mut nxt_cur = cur.clone();
            nxt_cur.push(rem[i]);
            let mut nxt_rem = rem.clone();
            nxt_rem.remove(i);
            Solution::r(hs, nxt_cur.clone(), nxt_rem);
            hs.insert(nxt_cur.into_iter().collect());
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut res = HashSet::<String>::new();
        let tiles_c: Vec<char> = tiles.chars().collect();
        Solution::r(&mut res, Vec::new(), tiles_c);
        res.len() as i32
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
