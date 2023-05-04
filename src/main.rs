mod utils;

use std::{collections::VecDeque, iter::FromIterator};
use utils::Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        const R: u8 = b'R';
        const D: u8 = b'D';
        let mut rcount = 0;
        let mut dcount = 0;
        let mut dfloating = 0;
        let mut rfloating = 0;

        let mut queue = VecDeque::from_iter(senate.as_bytes().iter().cloned());
        for &c in queue.iter() {
            if c == R {
                rcount += 1;
            } else {
                dcount += 1;
            }
        }

        while (rcount > 0) && (dcount > 0) {
            let curr = queue.pop_front().unwrap();

            if curr == D {
                if dfloating > 0 {
                    dfloating -= 1;
                    dcount -= 1;
                } else {
                    rfloating += 1;
                    queue.push_back(D);
                }
            } else {
                if rfloating > 0 {
                    rfloating -= 1;
                    rcount -= 1;
                } else {
                    dfloating += 1;
                    queue.push_back(R);
                }
            }
        }

        let result = if rcount > 0 { "Radiant" } else { "Dire" };
        return result.to_string();
    }
}

fn main() {
    let inputs = ["RD", "RDD"];

    for senate in inputs {
        let result = Solution::predict_party_victory(senate.to_string());
        println!("{result}");
    }
}
