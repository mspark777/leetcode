"""
main
"""

from typing import Optional
from solution import Solution, TreeNode


class Input:
    root: Optional[TreeNode]
    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs = [
            Input(
                TreeNode(1,
                    None,
                    TreeNode(2,
                        TreeNode(3, None, None),
                        None
                    ),
                ),
            ),
            Input(None),
            Input(TreeNode(1, None, None)),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.preorderTraversal(i.root)
        print(result)



if __name__ == "__main__":
    main()
