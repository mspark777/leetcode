from __future__ import annotations
from typing import Optional, List


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def newnode(val: int, left: Optional[TreeNode], right: Optional[TreeNode]) -> TreeNode:
    return TreeNode(val, left, right)


def newleft(val: int, left: Optional[TreeNode]) -> TreeNode:
    return newnode(val, left, None)


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return newnode(val, None, right)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


def treetoarr(node: Optional[TreeNode]) -> list[int]:
    nums: list[int] = []

    def travel(n: Optional[TreeNode]):
        if n is not None:
            nums.append(n.val)
            travel(n.left)
            travel(n.right)

    travel(node)
    return nums


class Solution:
    def addOneRow(
        self, root: Optional[TreeNode], val: int, depth: int
    ) -> Optional[TreeNode]:
        if root is None:
            return None

        if depth == 1:
            return TreeNode(val, root)

        stack: list[tuple[TreeNode, int]] = [(root, 2)]
        targets: list[TreeNode] = []
        while stack:
            (node, pos) = stack.pop()
            if pos > depth:
                continue

            if pos == depth:
                targets.append(node)
                continue

            left = node.left
            if left is not None:
                stack.append((left, pos + 1))
            right = node.right
            if right is not None:
                stack.append((right, pos + 1))

        for target in targets:
            target.left = TreeNode(val, target.left)
            target.right = TreeNode(val, None, target.right)

        return root


class Input:
    root: Optional[TreeNode]
    val: int
    depth: int

    def __init__(self, root: Optional[TreeNode], val: int, depth: int) -> None:
        self.root = root
        self.val = val
        self.depth = depth


def main():
    inputs: list[Input] = [
        Input(
            newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))), 1, 2
        ),
        Input(newleft(4, newnode(2, newval(3), newval(1))), 1, 3),
        Input(
            newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))), 1, 1
        ),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        val = input.val
        depth = input.depth
        result = solution.addOneRow(root, val, depth)
        print(treetoarr(result))


if __name__ == "__main__":
    main()
