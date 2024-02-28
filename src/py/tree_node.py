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


def new_tree_node(
    val: int, left: Optional[TreeNode], right: Optional[TreeNode]
) -> TreeNode:
    return TreeNode(val, left, right)


def new_tree_left(val: int, left: TreeNode) -> TreeNode:
    return new_tree_node(val, left, None)


def new_tree_right(val: int, right: TreeNode) -> TreeNode:
    return new_tree_node(val, None, right)


def new_tree_val(val: int) -> TreeNode:
    return new_tree_node(val, None, None)


def to_tree(items: list[Optional[int]]) -> Optional[TreeNode]:
    if not items:
        return None

    it = iter(items)
    val = next(it)
    if val is None:
        return None

    root = TreeNode(val)
    q = [root]
    for node in q:
        val = next(it, None)
        if val is not None:
            node.left = TreeNode(val)
            q.append(node.left)
        val = next(it, None)
        if val is not None:
            node.right = TreeNode(val)
            q.append(node.right)
    return root
