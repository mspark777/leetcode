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


class Solution:
    def buildTree(self, inorder: list[int], postorder: list[int]) -> Optional[TreeNode]:
        return self.build(
            inorder, postorder, 0, len(inorder) - 1, 0, len(postorder) - 1
        )

    def build(
        self,
        inorder: list[int],
        postorder: list[int],
        inLeft: int,
        inRight: int,
        postLeft: int,
        postRight: int,
    ) -> Optional[TreeNode]:
        if (inLeft > inRight) or (postLeft > postRight):
            return None

        val = postorder[postRight]
        node = TreeNode(val)
        idx = inorder.index(val)

        leftSize = idx - inLeft
        rightSize = inRight - idx
        node.left = self.build(
            inorder, postorder, inLeft, idx - 1, postLeft, postLeft + leftSize - 1
        )
        node.right = self.build(
            inorder, postorder, idx + 1, inRight, postRight - rightSize, postRight - 1
        )

        return node


def main():
    inputs = [([9, 3, 15, 20, 7], [9, 15, 7, 20, 3]), ([-1], [-1])]

    for inorder, postorder in inputs:
        solution = Solution()
        result = solution.buildTree(inorder, postorder)
        print(result)


if __name__ == "__main__":
    main()
