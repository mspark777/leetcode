from typing import Optional

from lib import new_tree_node, new_tree_val, TreeNode


class Solution:
    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        return self.travel(root, 0)

    def travel(self, node: Optional[TreeNode], sum: int) -> int:
        if node is None:
            return 0

        newsum = (sum * 10) + node.val
        left = node.left
        right = node.right

        if (left is None) and (right is None):
            return newsum

        return self.travel(left, newsum) + self.travel(right, newsum)


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_node(1, new_tree_val(2), new_tree_val(3)),
        new_tree_node(
            4, new_tree_node(9, new_tree_val(5), new_tree_val(1)), new_tree_val(0)
        ),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.sumNumbers(root)
        print(result)


if __name__ == "__main__":
    main()
