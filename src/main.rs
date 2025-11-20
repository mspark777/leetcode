struct Solution {}

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1_set = std::collections::HashSet::<i32>::from_iter(nums1.iter().cloned());
        let nums2_set = std::collections::HashSet::<i32>::from_iter(nums2.iter().cloned());

        if let Some(m) = nums1_set.intersection(&nums2_set).min() {
            return *m;
        }

        let min1 = nums1.iter().cloned().min().unwrap();
        let min2 = nums2.iter().cloned().min().unwrap();

        if min1 < min2 {
            min1 * 10 + min2
        } else {
            min2 * 10 + min1
        }
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums1: [4, 1, 3].to_vec(),
            nums2: [5, 7].to_vec(),
        },
        Input {
            nums1: [3, 5, 2, 6].to_vec(),
            nums2: [3, 1, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_number(input.nums1, input.nums2);
        println!("{:?}", result);
    }
}
