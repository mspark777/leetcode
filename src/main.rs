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

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = Vec::<[i32; 2]>::new();
        let mut counter = 0;
        let mut cur = head.as_ref();
        while cur.is_some() {
            let val = cur.as_ref().unwrap().val;
            while !stack.is_empty() && (stack.last().unwrap()[1] < val) {
                res[stack.last().unwrap()[0] as usize] = val;
                stack.pop();
            }
            cur = cur.as_ref().unwrap().next.as_ref();
            stack.push([counter, val]);
            res.push(0);
            counter += 1;
        }
        return res;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
