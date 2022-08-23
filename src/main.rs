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

struct Solution {}
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut node = &head;
        let mut nums = Vec::<i32>::new();
        while let Some(n) = node {
            nums.push(n.val);
            node = &n.next;
        }

        if nums.is_empty() {
            return true;
        }

        let mut i = 0usize;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[i] != nums[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

struct Input {
    nums: Vec<i32>,
}

impl Input {
    fn to_list(&self) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(self.nums[0])));
        let mut current = head.as_mut();

        for i in 1..self.nums.len() {
            if let Some(cur) = current {
                cur.next = Some(Box::new(ListNode::new(self.nums[i])));
                current = cur.next.as_mut();
            }
        }

        head
    }
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 2, 1],
        },
        Input { nums: vec![1, 2] },
    ];

    for input in inputs.iter() {
        let result = Solution::is_palindrome(input.to_list());
        println!("{}", result);
    }
}
