struct Solution {}

impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let a1 = nums1
            .iter()
            .cloned()
            .filter(|&n| n < 0)
            .map(|n| -n)
            .rev()
            .collect::<Vec<_>>();
        let a2 = nums1
            .iter()
            .cloned()
            .filter(|&n| n >= 0)
            .collect::<Vec<_>>();
        let mut b1 = nums2
            .iter()
            .cloned()
            .filter(|&n| n < 0)
            .map(|n| -n)
            .rev()
            .collect::<Vec<_>>();
        let mut b2 = nums2
            .iter()
            .cloned()
            .filter(|&n| n >= 0)
            .collect::<Vec<_>>();
        let mut k = k as usize;
        let neg = a1.len() * b2.len() + a2.len() * b1.len();
        let s = if k > neg {
            k -= neg;
            1i64
        } else {
            k = neg - k + 1;
            std::mem::swap(&mut b1, &mut b2);
            -1i64
        };

        let mut left = 0i64;
        let mut right = 10i64.pow(10);
        while left < right {
            let mid = (left + right) / 2;
            if (Self::count(&a1, &b1, mid) + Self::count(&a2, &b2, mid)) >= k {
                right = mid
            } else {
                left = mid + 1;
            }
        }

        return left * s;
    }

    fn count(a: &Vec<i32>, b: &Vec<i32>, x: i64) -> usize {
        let mut res = 0usize;
        let mut j = (b.len() as i32) - 1;
        for n in a.iter().cloned() {
            while j >= 0 && (n as i64) * (b[j as usize] as i64) > x {
                j -= 1;
            }

            res += (j + 1) as usize;
        }
        return res;
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    k: i64,
}

fn main() {
    let inputs = vec![
        Input {
            nums1: [2, 5].to_vec(),
            nums2: [3, 4].to_vec(),
            k: 2,
        },
        Input {
            nums1: [-4, -2, 0, 3].to_vec(),
            nums2: [2, 4].to_vec(),
            k: 6,
        },
        Input {
            nums1: [-2, -1, 0, 1, 2].to_vec(),
            nums2: [-3, -1, 2, 4, 5].to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::kth_smallest_product(input.nums1, input.nums2, input.k);
        println!("{:?}", result);
    }
}
