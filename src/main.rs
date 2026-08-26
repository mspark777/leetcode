struct Solution;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        for &a in &arr {
            hm.entry(a).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut v = hm.keys().cloned().collect::<Vec<_>>();
        v.sort_unstable_by_key(|k| k.abs());
        for k in v {
            let n1 = *hm.get(&k).unwrap();
            if n1 > 0 {
                if let Some(n2) = hm.get_mut(&(k * 2)) {
                    *n2 -= n1;
                    if *n2 < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [1, 2, 3, 4].to_vec(),
    }];

    for input in inputs {
        let result = Solution::can_reorder_doubled(input.arr);
        println!("{:?}", result);
    }
}
