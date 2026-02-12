struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for n1 in nums1 {
            for n2 in nums2.iter().copied() {
                let n = n2 * k;
                if (n1 % n) == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums1: [1, 3, 4].to_vec(),
            nums2: [1, 3, 4].to_vec(),
            k: 1,
        },
        Input {
            nums1: [1, 2, 4, 12].to_vec(),
            nums2: [2, 4].to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::number_of_pairs(input.nums1, input.nums2, input.k);
        println!("{:?}", result);
    }
}
