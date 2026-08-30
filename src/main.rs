struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(mut a: Vec<i32>, l: i32, m: i32) -> i32 {
        let l = l as usize;
        let m = m as usize;

        for i in 1..a.len() {
            a[i] += a[i - 1];
        }

        let mut res = a[l + m - 1];
        let mut lmax = a[l - 1];
        let mut mmax = a[m - 1];

        for i in (l + m)..a.len() {
            lmax = lmax.max(a[i - m] - a[i - l - m]);
            mmax = mmax.max(a[i - l] - a[i - l - m]);
            res = res.max(lmax + a[i] - a[i - m]).max(mmax + a[i] - a[i - l]);
        }

        return res;
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
