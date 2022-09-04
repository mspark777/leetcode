from __future__ import annotations
from typing import Optional


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def new_tree_node(
    val: int, left: Optional[TreeNode], right: Optional[TreeNode]
) -> TreeNode:
    return TreeNode(val, left, right)


def new_tree_val(val: int) -> TreeNode:
    return new_tree_node(val, None, None)


def new_tree_left(val: int, left: TreeNode) -> TreeNode:
    return new_tree_node(val, left, None)


def new_tree_right(val: int, right: TreeNode) -> TreeNode:
    return new_tree_node(val, None, right)
