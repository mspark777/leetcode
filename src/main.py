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


def newnode(val: int, left: TreeNode, right: TreeNode) -> TreeNode:
    return TreeNode(val, left, right)


def newleft(val: int, left: TreeNode) -> TreeNode:
    return TreeNode(val, left)


def newright(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, None, right)


def newval(val: int) -> TreeNode:
    return TreeNode(val)


class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        stack1: list[int] = []
        stack2: list[int] = []

        self.dfs(stack1, root1)
        self.dfs(stack2, root2)

        return stack1 == stack2

    def dfs(self, stack: list[int], node: Optional[TreeNode]) -> None:
        if node is None:
            return

        val = node.val
        left = node.left
        right = node.right

        if (left is None) and (right is None):
            stack.append(val)

        self.dfs(stack, left)
        self.dfs(stack, right)


def main():
    inputs: list[tuple[Optional[TreeNode],Optional[TreeNode]]] = [
        (
       newnode(3,
        newnode(5,
          newval(6),
          newnode(2,
            newval(7),
            newval(4)
          )
        ),
        newnode(1,
          newval(9),
          newval(8)
        )
      ),
       newnode(3,
        newnode(5, newval(6), newval(7)),
        newnode(1,
          newval(4),
          newnode(2,
            newval(9),
            newval(8)
          )
        )
      )
        ),
        (
       newnode(1, newval(2), newval(3)),
       newnode(1, newval(3), newval(2))
        ),
    ]

    solution = Solution()
    for root1, root2 in inputs:
        result = solution.leafSimilar(root1, root2)
        print(result)


if __name__ == "__main__":
    main()
