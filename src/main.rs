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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while let Some(fnode) = fast {
            if let Some(ffnode) = &fnode.next {
                slow = slow.unwrap().next.as_ref();
                fast = ffnode.next.as_ref();
            } else {
                break;
            }
        }

        if let Some(s) = slow {
            return Some(s.clone());
        } else {
            return None;
        }
    }
}

fn arrtolist(nums: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut tail = &mut head;

    for val in nums.iter().cloned() {
        tail.next = Some(Box::new(ListNode::new(val)));
        tail = tail.next.as_mut().unwrap();
    }

    return head.next;
}

fn listtoarr(node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums = Vec::<i32>::new();

    let mut n = node;
    while let Some(nn) = n {
        nums.push(nn.val);
        n = &nn.next;
    }

    return nums;
}

fn main() {
    let inputs = [vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5, 6]];

    for nums in inputs {
        let result = Solution::middle_node(arrtolist(&nums));
        let result = listtoarr(&result);
        println!("{result:?}");
    }
}
