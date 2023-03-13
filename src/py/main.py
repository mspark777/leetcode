from typing import Optional

from lib import new_tree_node, new_tree_right, new_tree_val, TreeNode


class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        return self.is_mirror(root.left, root.right) if root is not None else True

    def is_mirror(self, left: Optional[TreeNode], right: Optional[TreeNode]) -> bool:
        if (left is None) and (right is None):
            return True
        elif (left is None) or (right is None):
            return False

        return (
            (left.val == right.val)
            and self.is_mirror(left.left, right.right)
            and self.is_mirror(left.right, right.left)
        )


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_node(
            1,
            new_tree_node(2, new_tree_val(3), new_tree_val(4)),
            new_tree_node(2, new_tree_val(4), new_tree_val(3)),
        ),
        new_tree_node(
            1, new_tree_right(2, new_tree_val(3)), new_tree_right(2, new_tree_val(3))
        ),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.isSymmetric(root)
        print(result)


if __name__ == "__main__":
    main()
