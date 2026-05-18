struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        use std::collections::HashMap;

        let mut sums = HashMap::<i32, i32>::new();
        for n1 in nums1.iter().copied() {
            for n2 in nums2.iter().copied() {
                let sum = n1 + n2;
                sums.entry(sum).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let mut result = 0;
        for n3 in nums3.iter().copied() {
            for n4 in nums4.iter().copied() {
                let target = -(n3 + n4);
                result += sums.get(&target).copied().unwrap_or_default()
            }
        }

        result
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums3: Vec<i32>,
    nums4: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums1: [1, 2].to_vec(),
            nums2: [-2, -1].to_vec(),
            nums3: [-1, 2].to_vec(),
            nums4: [0, 2].to_vec(),
        },
        Input {
            nums1: [0].to_vec(),
            nums2: [0].to_vec(),
            nums3: [0].to_vec(),
            nums4: [0].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::four_sum_count(input.nums1, input.nums2, input.nums3, input.nums4);
        println!("{:?}", result);
    }
}
