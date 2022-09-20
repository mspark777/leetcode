struct Solution {}
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut lengths = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];

        for i in (0..nums1.len()).rev() {
            for j in (0..nums2.len()).rev() {
                if nums1[i] != nums2[j] {
                    continue;
                }

                let length = lengths[i + 1][j + 1] + 1;
                lengths[i][j] = length;
                result = result.max(length);
            }
        }

        result
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums1: vec![1, 2, 3, 2, 1],
            nums2: vec![3, 2, 1, 4, 7],
        },
        Input {
            nums1: vec![0, 0, 0, 0, 0],
            nums2: vec![0, 0, 0, 0, 0],
        },
    ];

    for Input { nums1, nums2 } in inputs.into_iter() {
        let result = Solution::find_length(nums1, nums2);
        println!("{result:?}");
    }
}
