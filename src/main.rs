struct Solution {}

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut pos1 = 0usize;
        let mut pos2 = 0usize;

        let mut result = Vec::<Vec<i32>>::new();
        while pos1 < n1 && pos2 < n2 {
            let id1 = nums1[pos1][0];
            let id2 = nums2[pos2][0];
            let val1 = nums1[pos1][1];
            let val2 = nums2[pos2][1];

            if id1 == id2 {
                result.push(vec![id1, val1 + val2]);
                pos1 += 1;
                pos2 += 1;
            } else if id1 < id2 {
                result.push(vec![id1, val1]);
                pos1 += 1;
            } else {
                result.push(vec![id2, val2]);
                pos2 += 1;
            }
        }

        for i in pos1..n1 {
            result.push(nums1[i].clone());
        }

        for i in pos2..n2 {
            result.push(nums2[i].clone());
        }

        return result;
    }
}

struct Input {
    nums1: Vec<Vec<i32>>,
    nums2: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums1: vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            nums2: vec![vec![1, 4], vec![3, 2], vec![4, 1]],
        },
        Input {
            nums1: vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            nums2: vec![vec![1, 3], vec![4, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::merge_arrays(input.nums1, input.nums2);
        println!("{result:?}");
    }
}
