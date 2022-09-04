"""
main
"""

from __future__ import annotations
from typing import Optional
from lib import TreeNode, new_tree_node, new_tree_left, new_tree_val


class StackNode:
    row: int
    col: int
    node: Optional[TreeNode]

    def __init__(self, row: int, col: int, node: Optional[TreeNode]):
        self.row = row
        self.col = col
        self.node = node


class VerticalNode:
    row: int
    col: int
    value: int

    def __init__(self, row: int, col: int, value: int) -> None:
        self.row = row
        self.col = col
        self.value = value


class ResultNode:
    col: int
    values: list[int]

    def __init__(self, col: int, values: list[int]) -> None:
        self.col = col
        self.values = values


class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> list[list[int]]:
        verticals: dict[int, list[VerticalNode]] = {}
        stack: list[StackNode] = [StackNode(0, 0, root)]

        while len(stack) > 0:
            top = stack.pop()
            if top.node is None:
                continue

            if top.col not in verticals:
                verticals[top.col] = []

            vertical = verticals[top.col]
            vertical.append(VerticalNode(top.row, top.col, top.node.val))

            stack.append(StackNode(top.row + 1, top.col - 1, top.node.left))
            stack.append(StackNode(top.row + 1, top.col + 1, top.node.right))

        result: list[ResultNode] = []
        for nodes in verticals.values():
            nodes.sort(key=lambda node: (node.row, node.value))
            rnode = ResultNode(nodes[0].col, [])
            for node in nodes:
                rnode.values.append(node.value)
            result.append(rnode)

        result.sort(key=lambda node: node.col)
        return [node.values for node in result]


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(
            new_tree_node(
                3, new_tree_val(9), new_tree_node(20, new_tree_val(15), new_tree_val(7))
            )
        ),
        Input(
            new_tree_node(
                1,
                new_tree_node(2, new_tree_val(4), new_tree_val(5)),
                new_tree_node(3, new_tree_val(6), new_tree_val(7)),
            )
        ),
        Input(
            new_tree_node(
                3,
                new_tree_node(1, new_tree_val(0), new_tree_val(2)),
                new_tree_left(4, new_tree_val(2)),
            )
        ),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.verticalTraversal(root)
        print(result)


if __name__ == "__main__":
    main()
