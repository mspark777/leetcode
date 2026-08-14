struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(nums);
        let mut result = 0;
        let mut current = &head;
        let mut inprev = false;

        while let Some(node) = current {
            let x = set.contains(&node.val);
            if x && !inprev {
                result += 1;
            }
            inprev = x;
            current = &node.next;
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "(123)".to_string(),
    }];

    for input in inputs {
        let result = Solution::ambiguous_coordinates(input.s);
        println!("{:?}", result);
    }
}
