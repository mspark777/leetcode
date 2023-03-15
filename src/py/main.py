from typing import Optional

from lib import new_tree_node, new_tree_right, new_tree_left, new_tree_val, TreeNode


class Solution:
    def isCompleteTree(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        null_found = False
        queue: list[Optional[TreeNode]] = [root]

        while queue:
            node = queue.pop(0)

            if node is None:
                null_found = True
                continue

            if null_found:
                return False

            queue.append(node.left)
            queue.append(node.right)

        return True


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_node(
            1,
            new_tree_node(2, new_tree_val(4), new_tree_val(5)),
            new_tree_left(3, new_tree_val(6)),
        ),
        new_tree_node(
            1,
            new_tree_node(2, new_tree_val(4), new_tree_val(5)),
            new_tree_right(3, new_tree_val(7)),
        ),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.isCompleteTree(root)
        print(result)


if __name__ == "__main__":
    main()
