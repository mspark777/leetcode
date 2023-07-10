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


def newnode(val: int, left: Optional[TreeNode], right: Optional[TreeNode]) -> TreeNode:
    return TreeNode(val, left, right)


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return TreeNode(val, None, right)


def newval(val: int) -> TreeNode:
    return TreeNode(val)


class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        queue: list[TreeNode] = [root]
        depth = 0
        while queue:
            depth += 1
            count = len(queue)
            for i in range(count):
                node = queue[i]
                found = True
                if node.left is not None:
                    found = False
                    queue.append(node.left)

                if node.right is not None:
                    found = False
                    queue.append(node.right)

                if found:
                    return depth
            queue = queue[count:]

        return depth


def main():
    inputs = [
        newnode(3, newval(9), newnode(20, newval(15), newval(7))),
        newright(2, newright(3, newright(4, newright(5, newval(6))))),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.minDepth(root)
        print(result)


if __name__ == "__main__":
    main()
