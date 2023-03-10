from __future__ import annotations
from typing import Optional


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(
        self,
        val: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
        self.val = val
        self.left = left
        self.right = right


def new_tree_node(val: int, left: TreeNode, right: TreeNode) -> TreeNode:
    return TreeNode(val, left, right)


def new_tree_left(val: int, left: TreeNode) -> TreeNode:
    return TreeNode(val, left)


def new_tree_right(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, right=right)


def new_tree_val(val: int) -> TreeNode:
    return TreeNode(val)


class ListNode:
    x: int
    next: Optional[ListNode]

    def __init__(self, x: int, next: Optional[ListNode]):
        self.val = x
        self.next = next


def new_list_node(val: int, next: Optional[ListNode]) -> ListNode:
    return ListNode(val, next)


def new_list(vals: list[int]) -> Optional[ListNode]:
    head = new_list_node(0, None)
    tail = head

    for val in vals:
        node = new_list_node(val, None)
        tail.next = node
        tail = node

    return head.next


def new_cycle_list(vals: list[int], pos: int) -> Optional[ListNode]:
    head = new_list_node(0, None)
    tail = head
    target: Optional[ListNode] = None

    for i in range(len(vals)):
        node = new_list_node(vals[i], None)
        if i == pos:
            target = node

        tail.next = node
        tail = tail.next

    tail.next = target
    return head.next
