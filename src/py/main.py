from typing import Optional

from utils import TreeNode, new_tree_right, new_tree_node, new_tree_val


class Solution:
    max_step: int

    def __init__(self):
        self.max_step = 0

    def dfs(self, node: Optional[TreeNode], left: bool, steps: int):
        if node is None:
            return

        self.max_step = max(self.max_step, steps)
        if left:
            self.dfs(node.left, False, steps + 1)
            self.dfs(node.right, True, 1)
        else:
            self.dfs(node.left, False, 1)
            self.dfs(node.right, True, steps + 1)

    def longestZigZag(self, root: Optional[TreeNode]) -> int:
        self.dfs(root, True, 0)
        self.dfs(root, False, 0)

        return self.max_step


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_right(
            1,
            new_tree_node(
                1,
                new_tree_val(1),
                new_tree_node(
                    1,
                    new_tree_right(1, new_tree_right(1, new_tree_val(1))),
                    new_tree_val(1),
                ),
            ),
        ),
        new_tree_node(
            1,
            new_tree_right(
                1, new_tree_node(1, new_tree_right(1, new_tree_val(1)), new_tree_val(1))
            ),
            new_tree_val(1),
        ),
        new_tree_val(1),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.longestZigZag(root)
        print(result)


if __name__ == "__main__":
    main()
