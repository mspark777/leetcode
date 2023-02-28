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
