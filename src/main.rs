struct Solution;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        vec![Self::count(&nums1, &nums2), Self::count(&nums2, &nums1)]
    }

    fn count(left: &[i32], right: &[i32]) -> i32 {
        use std::collections::HashSet;

        let set = HashSet::<i32>::from_iter(right.iter().copied());
        let mut count = 0;
        for num in left {
            if set.contains(num) {
                count += 1;
            }
        }

        count
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums1: [2, 3, 2].to_vec(),
            nums2: [1, 2].to_vec(),
        },
        Input {
            nums1: [4, 3, 2, 3, 1].to_vec(),
            nums2: [2, 2, 5, 2, 3, 6].to_vec(),
        },
        Input {
            nums1: [3, 4, 2, 3].to_vec(),
            nums2: [1, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_intersection_values(input.nums1, input.nums2);
        println!("{:?}", result);
    }
}
