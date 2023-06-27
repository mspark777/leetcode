mod utils;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use utils::Solution;

#[derive(PartialEq, Eq)]
struct Node {
    sum: i32,
    i1: usize,
    i2: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return if self.sum < other.sum {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut k = k as usize;
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut result = Vec::<Vec<i32>>::with_capacity(k);
        let mut visited = HashSet::<(usize, usize)>::with_capacity(len1 + len2);
        let mut queue = BinaryHeap::<Node>::with_capacity(visited.capacity());

        queue.push(Node {
            sum: nums1[0] + nums2[0],
            i1: 0,
            i2: 0,
        });
        visited.insert((0, 0));

        while let Some(node) = queue.pop() {
            let i1 = node.i1;
            let i2 = node.i2;
            result.push(vec![nums1[i1], nums2[i2]]);

            let i3 = i1 + 1;
            if (i3 < len1) && !visited.contains(&(i3, i2)) {
                queue.push(Node {
                    sum: nums1[i3] + nums2[i2],
                    i1: i3,
                    i2,
                });
                visited.insert((i3, i2));
            }

            let i4 = i2 + 1;
            if (i4 < len2) && !visited.contains(&(i1, i4)) {
                queue.push(Node {
                    sum: nums1[i1] + nums2[i4],
                    i1,
                    i2: i4,
                });
                visited.insert((i1, i4));
            }

            if k > 1 {
                k -= 1;
            } else {
                break;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 7, 11], vec![2, 4, 6], 3),
        (vec![1, 1, 2], vec![1, 2, 3], 2),
        (vec![1, 2], vec![3], 3),
    ];

    for (nums1, nums2, k) in inputs {
        let result = Solution::k_smallest_pairs(nums1, nums2, k);
        println!("{result:?}");
    }
}
