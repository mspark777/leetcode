#[derive(Clone, Copy)]
struct Pair {
    n: i32,
    i: usize,
}

fn merge(pairs: &mut Vec<Pair>, l: usize, mid: usize, r: usize, result: &mut Vec<i32>) {
    let mut i = l;
    let mut j = mid + 1;
    let mut temp = Vec::<Pair>::with_capacity(r - l + 1);
    let mut count = 0;

    while (i <= mid) && (j <= r) {
        let ip = &pairs[i];
        let jp = &pairs[j];
        if ip.n <= jp.n {
            result[ip.i] += count;
            temp.push(*ip);

            i += 1;
        } else {
            count += 1;
            temp.push(*jp);
            j += 1;
        }
    }

    while i <= mid {
        let p = &pairs[i];
        result[p.i] += count;
        temp.push(*p);
        i += 1;
    }

    while j <= r {
        temp.push(pairs[j]);
        j += 1;
    }

    for (i2, p) in temp.iter().enumerate() {
        pairs[l + i2] = *p;
    }
}

fn mergesort(pairs: &mut Vec<Pair>, l: usize, r: usize, result: &mut Vec<i32>) {
    if l >= r {
        return;
    }

    let mid = (l + r) / 2;
    mergesort(pairs, l, mid, result);
    mergesort(pairs, mid + 1, r, result);
    merge(pairs, l, mid, r, result)
}

pub struct Solution {}
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut pairs = Vec::<Pair>::with_capacity(n);
        for i in 0..n {
            let p = Pair { n: nums[i], i };
            pairs.push(p);
        }

        mergesort(&mut pairs, 0, n - 1, &mut result);
        return result;
    }
}
