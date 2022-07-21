#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution {}
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut l = &head;
        let mut r = &head;
        for _ in 0..left {
            if let Some(ln) = l {
                l = &ln.next;
                r = &ln.next;
            }
        }

        for _ in left..right {
            if let Some(rn) = r {
                r = &rn.next;
            }
        }

        head
    }
}
