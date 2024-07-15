from __future__ import annotations
from typing import List, Optional
from tree_node import TreeNode


class Solution:
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        node_map = dict[int, TreeNode]()
        children = set[int]()

        for description in descriptions:
            parent_value = description[0]
            child_value = description[1]
            children.add(child_value)

            parent = node_map.get(parent_value)
            if parent is None:
                parent = TreeNode(parent_value)
                node_map[parent_value] = parent

            if child_value not in node_map:
                node_map[child_value] = TreeNode(child_value)

            is_left = description[2] == 1

            if is_left:
                parent.left = node_map[child_value]
            else:
                parent.right = node_map[child_value]

        for node in node_map.values():
            if node.val not in children:
                return node

        return None


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
