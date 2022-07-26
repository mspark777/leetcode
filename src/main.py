"""
main
"""

from typing import Optional
from solution import Solution, TreeNode

def find_node(node: Optional[TreeNode], val: int) -> Optional[TreeNode]:
    if node is None:
        return node
    elif node.val == val:
        return node

    left = find_node(node.left, val)
    return left if left is not None else find_node(node.right, val)


class Input:
    root: Optional[TreeNode]
    p: Optional[TreeNode]
    q: Optional[TreeNode]
    def __init__(self, root: Optional[TreeNode], pv: int, qv: int):
        self.root = root
        self.p = find_node(root, pv)
        self.q = find_node(root, qv)


def main():
    inputs = [
            Input(
                TreeNode(3,
                    TreeNode(5,
                        TreeNode(6, None, None),
                        TreeNode(2,
                            TreeNode(7, None, None),
                            TreeNode(4, None, None)
                            )
                        ),
                    TreeNode(1,
                        TreeNode(0, None, None),
                        TreeNode(8, None, None)
                        )
                    ),
                5,
                1
                ),
            Input(
                TreeNode(3,
                    TreeNode(5,
                        TreeNode(6, None, None),
                        TreeNode(2,
                            TreeNode(7, None, None),
                            TreeNode(4, None, None)
                            )
                        ),
                    TreeNode(1,
                        TreeNode(0, None, None),
                        TreeNode(8, None, None)
                        )
                    ),
                5,
                4
                ),
            Input(
                TreeNode(1, TreeNode(2, None, None), None),
                1,
                2
                ),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.lowestCommonAncestor(i.root, i.p, i.q)
        print(result.val if result is not None else None)



if __name__ == "__main__":
    main()
