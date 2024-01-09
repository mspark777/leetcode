from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        list1 = self.to_list(root1, [])
        list2 = self.to_list(root2, [])

        return list1 == list2

    def to_list(self, node: Optional[TreeNode], result: list[int]) -> list[int]:
        if node is None:
            return result

        self.to_list(node.left, result)
        self.to_list(node.right, result)
        if node.left is None and node.right is None:
            result.append(node.val)

        return result


def main():
    input = ()

    for root1, root2 in input:
        result = Solution().leafSimilar(root1, root2)
        print(result)


if __name__ == "__main__":
    main()
