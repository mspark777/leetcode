from typing import Optional, List

from lib import TreeNode, new_tree_left, new_tree_node, new_tree_val


class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[TreeNode]:
        result: list[TreeNode] = []
        self.traverse(root, {}, {}, result)
        return result

    def traverse(
        self,
        node: Optional[TreeNode],
        triplets: dict[str, int],
        counts: dict[int, int],
        result: list[TreeNode],
    ) -> int:
        if node is None:
            return 0

        triplet = ",".join(
            [
                str(self.traverse(node.left, triplets, counts, result)),
                str(node.val),
                str(self.traverse(node.right, triplets, counts, result)),
            ]
        )

        if triplet not in triplets:
            triplets[triplet] = len(triplets) + 1

        id = triplets[triplet]
        counts[id] = counts.get(id, 0) + 1
        if counts.get(id) == 2:
            result.append(node)

        return id


def main():
    inputs: list[Optional[TreeNode]] = [
        new_tree_node(
            1,
            new_tree_left(2, new_tree_val(4)),
            new_tree_node(3, new_tree_left(2, new_tree_val(4)), new_tree_val(4)),
        ),
        new_tree_node(2, new_tree_val(1), new_tree_val(1)),
        new_tree_node(
            2, new_tree_left(2, new_tree_val(3)), new_tree_left(2, new_tree_val(3))
        ),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.findDuplicateSubtrees(root)
        print(result)


if __name__ == "__main__":
    main()
