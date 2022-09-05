"""
main
"""

from __future__ import annotations
from typing import Optional


class Node:
    val: int
    children: Optional[list[Node]]

    def __init__(self, val: int, children=None):
        self.val = val
        self.children = children


def newnode(val: int, children: Optional[list[Node]] = None) -> Node:
    return Node(val, children)


class Solution:
    def levelOrder(self, root: Optional[Node]) -> list[list[int]]:
        if root is None:
            return []

        queue: list[Node] = [root]
        result: list[list[int]] = []
        while len(queue) > 0:
            size = len(queue)
            values: list[int] = []
            for i in range(size):
                node = queue[i]
                values.append(node.val)
                if node.children is not None:
                    queue.extend(node.children)
            result.append(values)
            queue = queue[size:]
        return result


class Input:
    root: Optional[Node]

    def __init__(self, root: Optional[Node]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(
            newnode(1, [newnode(3, [newnode(5), newnode(6)]), newnode(2), newnode(4)])
        ),
        Input(
            newnode(
                1,
                [
                    newnode(2),
                    newnode(3, [newnode(6), newnode(7, [newnode(11, [newnode(14)])])]),
                    newnode(4, [newnode(8, [newnode(12)])]),
                    newnode(5, [newnode(9, [newnode(13)]), newnode(10)]),
                ],
            )
        ),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.levelOrder(root)
        print(result)


if __name__ == "__main__":
    main()
