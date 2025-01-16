struct Solution {}

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut xor1 = 0u32;
        let mut xor2 = 0u32;

        if nums2.len() & 1 == 1 {
            for num in nums1.iter().cloned() {
                xor1 ^= num as u32;
            }
        }

        if nums1.len() & 1 == 1 {
            for num in nums2.iter().cloned() {
                xor2 ^= num as u32;
            }
        }

        return (xor1 ^ xor2) as i32;
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums1: vec![2, 1, 3],
            nums2: vec![10, 2, 5, 0],
        },
        Input {
            nums1: vec![1, 2],
            nums2: vec![3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::xor_all_nums(input.nums1, input.nums2);
        println!("{result:?}");
    }
}
