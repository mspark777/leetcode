"""
main
"""

from typing import Optional
from solution import Solution, TreeNode


class Input:
    root: Optional[TreeNode]
    def __init__(self, root: Optional[TreeNode]):
        self.root = root

def treetoarr(node: Optional[TreeNode]):
    nums = []
    while node is not None:
        nums.append(node.val)
        node = node.right
    return nums

def main():
    inputs = [
            Input(
                TreeNode(1,
                    TreeNode(2,
                        TreeNode(3, None, None),
                        TreeNode(4, None, None)
                    ),
                    TreeNode(5,
                        None,
                        TreeNode(8, None, None)
                    )
                )
            ),
            Input(None),
            Input(TreeNode(0, None, None)),
    ]
    sol = Solution()
    for i in inputs:
        sol.flatten(i.root)
        print(treetoarr(i.root))



if __name__ == "__main__":
    main()
