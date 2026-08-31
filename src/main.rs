struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut barcodes = barcodes;
        let mut pos = 0;
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut s = std::collections::BTreeSet::<(i32, i32)>::new();

        for n in barcodes.iter() {
            *m.entry(*n).and_modify(|n| *n += 1).or_insert(1);
        }

        for (&k, &v) in m.iter() {
            s.insert((v, k));
        }

        for (v, k) in s.into_iter().rev() {
            for _ in 0..v {
                if pos >= barcodes.len() {
                    pos = 1;
                }
                barcodes[pos] = k;
                pos += 2;
            }
        }
        barcodes
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
