mod solution;

use solution::{ListNode, Solution};

struct Input {
    head: Vec<i32>,
    left: i32,
    right: i32,
}

fn arrtolist(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let head = Some(Box::new(ListNode { val: 0, next: None }));
    let mut tail = &head;
    for n in arr {
        let node = Some(Box::new(ListNode { val: n, next: None }));
        if let Some(t) = tail {
            let mut t = &mut t;
            t.next = node;
        }
    }

    head.unwrap().next
}

fn listtoarr(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut node = node;
    let mut arr = Vec::<i32>::new();
    while let Some(n) = node {
        arr.push(n.val);
        node = n.next
    }

    arr
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            head: vec![1, 2, 3, 4, 5],
            left: 2,
            right: 4,
        },
        Input {
            head: vec![5],
            left: 1,
            right: 1,
        },
        Input {
            head: vec![],
            left: 1,
            right: 100,
        },
    ];

    for input in inputs {
        let result = Solution::reverse_between(arrtolist(input.head), input.left, input.right);
        let result = listtoarr(result);
        println!("{result:?}");
    }
}
