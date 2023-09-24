from __future__ import annotations
from typing import Optional, List


class Node:
    val: int
    left: Optional[Node]
    right: Optional[Node]
    next: Optional[Node]

    def __init__(
        self,
        val: int = 0,
        left: Optional[Node] = None,
        right: Optional[Node] = None,
        next: Optional[Node] = None,
    ):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def connect(self, root: Optional[Node]) -> Optional[Node]:
        if root is None:
            return None
        left = root.left
        right = root.right
        next = root.next
        if left is not None:
            left.next = right
            if (next is not None) and (right is not None):
                right.next = next.left
            self.connect(left)
            self.connect(right)

        return root


def main():
    inputs = [Node(1, Node(2, Node(4), Node(5)), Node(3, Node(6), Node(7))), None]

    for root in inputs:
        solution = Solution()
        result = solution.connect(root)
        print(result)


if __name__ == "__main__":
    main()
