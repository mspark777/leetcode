from __future__ import annotations
from typing import Optional, List


class Node:
    val: int
    neighbors: list[Node]

    def __init__(self, val: int = 0, neighbors: Optional[list[Node]] = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:
        if node is None:
            return None

        queue: list[Node] = [node]
        clones: dict[int, Node] = {node.val: Node(node.val, [])}

        while queue:
            cur = queue.pop(0)
            cur_clone = clones[cur.val]

            for ngbr in cur.neighbors:
                if ngbr.val not in clones:
                    clones[ngbr.val] = Node(ngbr.val, [])
                    queue.append(ngbr)

                cur_clone.neighbors.append(clones[ngbr.val])

        return clones[node.val]


def main():
    inputs = (
        [[2, 4], [1, 3], [2, 4], [1, 3]],
        [[]],
        [],
    )

    for node in inputs:
        result = Solution().validateBinaryTreeNodes(n, left, right)
        print(result)


if __name__ == "__main__":
    main()
