struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for (n1, n2) in nums1.iter().copied().zip(nums2.iter().copied()) {
            sum1 += n1;
            sum2 += n2;
        }

        let n = nums1.len() as i32;
        (sum2 - sum1) / n
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums1: [2, 6, 4].to_vec(),
            nums2: [9, 7, 5].to_vec(),
        },
        Input {
            nums1: [10].to_vec(),
            nums2: [5].to_vec(),
        },
        Input {
            nums1: [1, 1, 1, 1].to_vec(),
            nums2: [1, 1, 1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::added_integer(input.nums1, input.nums2);
        println!("{:?}", result);
    }
}
