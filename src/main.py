"""
main
"""

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

class Solution:
    def lowestCommonAncestor(self, root: Optional[TreeNode], p: Optional[TreeNode], q: Optional[TreeNode]) -> Optional[TreeNode]:
        if p is None or q is None:
            return None

        pval = p.val
        qval = q.val

        cur = root
        while cur is not None:
            val = cur.val
            if (pval < val) and (qval < val):
                cur = cur.left
            elif (pval > val) and (qval > val):
                cur = cur.right
            else:
                break
        return cur


class Input:
    root: Optional[TreeNode]
    p: Optional[TreeNode]
    q: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode], pval: int, qval: int):
        self.root = root
        self.p = self.get_node(root, pval)
        self.q = self.get_node(root, qval)

    def get_node(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        if root is None:
            return None

        if root.val == val:
            return root

        child = self.get_node(root.left, val)
        if child is not None:
            return child

        return self.get_node(root.right, val)

def main():
    inputs: list[Input] = [
            Input(
                TreeNode(6,
                    TreeNode(2,
                        TreeNode(0),
                        TreeNode(4,
                            TreeNode(3),
                            TreeNode(5)
                        )
                    ),
                    TreeNode(8,
                        TreeNode(7),
                        TreeNode(9)
                    )
                ),
                2,
                8
            ),
            Input(
                TreeNode(6,
                    TreeNode(2,
                        TreeNode(0),
                        TreeNode(4,
                            TreeNode(3),
                            TreeNode(5)
                        )
                    ),
                    TreeNode(8,
                        TreeNode(7),
                        TreeNode(9)
                    )
                ),
                2,
                4
            ),
            Input(
                TreeNode(2,
                    TreeNode(1)
                ),
                2,
                1
            ),
    ]

    s = Solution()
    for i in inputs:
        result = s.lowestCommonAncestor(i.root, i.p, i.q)
        if result is None:
            print(None)
        else:
            print(result.val)

if __name__ == "__main__":
    main()
