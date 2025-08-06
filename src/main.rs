struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        if n == 0 {
            return fruits.len() as i32;
        }

        let mut seg_node = vec![0; 4 * n + 7];
        let mut baskets = baskets;

        Self::build(&mut seg_node, &mut baskets, 1, 0, n - 1);
        let mut result = 0;
        for fruit in fruits.iter().cloned() {
            let mut l = 0usize;
            let mut r = n - 1;
            let mut m: Option<usize> = None;
            while l <= r {
                let mid = (l + r) / 2;
                if Self::query(&mut seg_node, &mut baskets, 1, 0, n - 1, 0, mid) >= fruit {
                    m = Some(mid);
                    if r > 0 {
                        r = mid - 1;
                    } else {
                        break;
                    }
                } else {
                    l = mid + 1;
                }
            }

            if let Some(pos) = m {
                if baskets[pos] >= fruit {
                    Self::update(&mut seg_node, &mut baskets, 1, 0, n - 1, pos, i32::MIN);
                    continue;
                }
            }
            result += 1;
        }

        result
    }

    fn build(seg_node: &mut Vec<i32>, baskets: &mut Vec<i32>, p: usize, l: usize, r: usize) {
        if l == r {
            seg_node[p] = baskets[l];
            return;
        }

        let mid = (l + r) / 2;
        Self::build(seg_node, baskets, p * 2, l, mid);
        Self::build(seg_node, baskets, p * 2 + 1, mid + 1, r);
        seg_node[p] = seg_node[p * 2].max(seg_node[p * 2 + 1]);
    }

    fn query(
        seg_node: &mut Vec<i32>,
        baskets: &mut Vec<i32>,
        p: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
    ) -> i32 {
        if ql > r || qr < l {
            return i32::MIN;
        }

        if ql <= l && r <= qr {
            return seg_node[p];
        }

        let mid = (l + r) / 2;
        Self::query(seg_node, baskets, p * 2, l, mid, ql, qr).max(Self::query(
            seg_node,
            baskets,
            p * 2 + 1,
            mid + 1,
            r,
            ql,
            qr,
        ))
    }

    fn update(
        seg_node: &mut Vec<i32>,
        baskets: &mut Vec<i32>,
        p: usize,
        l: usize,
        r: usize,
        pos: usize,
        val: i32,
    ) {
        if l == r {
            seg_node[p] = val;
            return;
        }

        let mid = (l + r) / 2;
        if pos <= mid {
            Self::update(seg_node, baskets, p * 2, l, mid, pos, val);
        } else {
            Self::update(seg_node, baskets, p * 2 + 1, mid + 1, r, pos, val);
        }
        seg_node[p] = seg_node[p * 2].max(seg_node[p * 2 + 1]);
    }
}

struct Input {
    fruits: Vec<i32>,
    baskets: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            fruits: [4, 2, 5].to_vec(),
            baskets: [3, 5, 4].to_vec(),
        },
        Input {
            fruits: [3, 6, 1].to_vec(),
            baskets: [6, 4, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::num_of_unplaced_fruits(input.fruits, input.baskets);
        println!("{:?}", result);
    }
}
