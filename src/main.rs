struct Solution {}

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1 = 0i64;
        let mut sum2 = 0i64;
        let mut zero1 = 0;
        let mut zero2 = 0;

        for n in nums1.iter().cloned() {
            sum1 += n as i64;
            if n == 0 {
                sum1 += 1;
                zero1 += 1;
            }
        }

        for n in nums2.iter().cloned() {
            sum2 += n as i64;
            if n == 0 {
                sum2 += 1;
                zero2 += 1;
            }
        }

        if ((zero1 == 0) && (sum2 > sum1)) || ((zero2 == 0) && (sum1 > sum2)) {
            return -1;
        }

        return sum1.max(sum2);
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums1: vec![3, 2, 0, 1, 0],
            nums2: vec![6, 5, 0],
        },
        Input {
            nums1: vec![2, 0, 2, 0],
            nums2: vec![1, 4],
        },
    ];

    for input in inputs {
        let result = Solution::min_sum(input.nums1, input.nums2);
        println!("{result:?}");
    }
}
