#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution {}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut cur = dummy.as_mut();

        while let Some(ncur) = cur.next.take() {
            if ncur.val == val {
                cur.next = ncur.next;
            } else {
                cur.next = Some(ncur);
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

struct Input {
    head: Option<Box<ListNode>>,
    val: i32,
}

fn arr_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut head;

    for num in nums {
        tail.next = Some(Box::new(ListNode {
            val: num,
            next: None,
        }));
        tail = tail.next.as_mut().unwrap();
    }

    head.next
}

fn list_to_arr(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut node = node;
    let mut nums = Vec::<i32>::new();

    while let Some(n) = node {
        nums.push(n.val);
        node = n.next;
    }

    nums
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            val: 6,
            head: arr_to_list(vec![1, 2, 6, 3, 4, 5, 6]),
        },
        Input {
            val: 1,
            head: arr_to_list(vec![]),
        },
        Input {
            val: 7,
            head: arr_to_list(vec![7, 7, 7, 7]),
        },
    ];

    for input in inputs {
        let val = input.val;
        let head = input.head;
        let result = Solution::remove_elements(head, val);
        let result = list_to_arr(result);
        println!("{:?}", result);
    }
}
