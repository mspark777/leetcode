from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def getDirections(
        self, root: Optional[TreeNode], start_value: int, dest_value: int
    ) -> str:
        start_path: list[str] = []
        dest_path: list[str] = []

        self.find_path(root, start_value, start_path)
        self.find_path(root, dest_value, dest_path)

        common_path_length = 0

        while True:
            if common_path_length >= len(start_path):
                break
            elif common_path_length >= len(dest_path):
                break
            elif start_path[common_path_length] != dest_path[common_path_length]:
                break
            else:
                common_path_length += 1

        directions: list[str] = []
        directions.extend("U" * (len(start_path) - common_path_length))
        directions.extend(dest_path[common_path_length:])

        return "".join(directions)

    def find_path(self, node: Optional[TreeNode], target: int, path: list[str]) -> bool:
        if node is None:
            return False

        if node.val == target:
            return True

        path.append("L")
        if self.find_path(node.left, target, path):
            return True
        path.pop()

        path.append("R")
        if self.find_path(node.right, target, path):
            return True
        path.pop()

        return False


def main():
    inputs: list[list[list[int]]] = [
        [[20, 15, 1], [20, 17, 0], [50, 20, 1], [50, 80, 0], [80, 19, 1]],
        [[1, 2, 1], [2, 3, 0], [3, 4, 1]],
    ]

    for input in inputs:
        result = Solution().createBinaryTree(input)
        print(result)


if __name__ == "__main__":
    main()
