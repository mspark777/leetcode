use utils::{Solution, UnionFind};

mod utils;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut result = strs.len() as i32;
        let mut uf = UnionFind::new(result);

        for (i, a) in strs.iter().enumerate() {
            for (j, b) in strs.iter().skip(i + 1).enumerate() {
                if !Self::is_similar(a, b) {
                    continue;
                }

                let x = i as i32;
                let y = (i + j + 1) as i32;
                if uf.find(x) != uf.find(y) {
                    result -= 1;
                    uf.union(x, y);
                }
            }
        }

        return result;
    }

    fn is_similar(a: &String, b: &String) -> bool {
        let mut diff = 0;
        let abytes = a.as_bytes();
        let bbytes = b.as_bytes();

        for i in 0..abytes.len() {
            if abytes[i] != bbytes[i] {
                diff += 1;
            }
        }

        return match diff {
            0 | 2 => true,
            _ => false,
        };
    }
}

fn main() {
    let inputs = [vec!["tars", "rats", "arts", "star"], vec!["omv", "ovm"]];

    for strs in inputs {
        let result = Solution::num_similar_groups(strs.iter().map(|s| s.to_string()).collect());
        println!("{result}");
    }
}
