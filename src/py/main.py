from typing import Optional

from utils import TreeNode, new_tree_right, new_tree_node, new_tree_val, new_tree_left


class Solution:
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        result = 1
        queue: list[tuple[TreeNode, int]] = [(root, 0)]
        while queue:
            count = len(queue)
            start = queue[0][1]
            end = queue[-1][1]
            result = max(result, end - start + 1)

            for i in range(count):
                node, node_idx = queue[i]
                idx = node_idx - start
                left = node.left
                right = node.right

                if left is not None:
                    queue.append((left, 2 * idx + 1))

                if right is not None:
                    queue.append((right, 2 * (idx + 1)))
            queue = queue[count:]

        return result


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_node(
            1,
            new_tree_node(3, new_tree_val(5), new_tree_val(3)),
            new_tree_right(2, new_tree_val(9)),
        ),
        new_tree_node(
            1,
            new_tree_left(3, new_tree_left(5, new_tree_val(6))),
            new_tree_right(2, new_tree_left(9, new_tree_val(7))),
        ),
        new_tree_node(1, new_tree_left(3, new_tree_val(5)), new_tree_val(2)),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.widthOfBinaryTree(root)
        print(result)


if __name__ == "__main__":
    main()
