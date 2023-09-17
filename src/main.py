from __future__ import annotations
from collections import deque


class Node:
    node: int
    mask: int
    cost: int

    def __init__(self, node: int, mask: int, cost: int):
        self.node = node
        self.mask = mask
        self.cost = cost


class Solution:
    def shortestPathLength(self, graph: list[list[int]]) -> int:
        node_count = len(graph)
        visited = set[tuple[int, int]]()
        all = (1 << node_count) - 1
        queue = deque[Node]()

        for i in range(node_count):
            mask = 1 << i
            queue.append(Node(i, mask, 1))
            visited.add((i, mask))

        while queue:
            curr = queue.popleft()
            if curr.mask == all:
                return curr.cost - 1

            for adj in graph[curr.node]:
                visited_mask = curr.mask | (1 << adj)
                this_node = Node(adj, visited_mask, curr.cost + 1)

                if (adj, visited_mask) not in visited:
                    visited.add((adj, visited_mask))
                    queue.append(this_node)

        return -1


def main():
    inputs = [[[1, 2, 3], [0], [0], [0]], [[1], [0, 2, 4], [1, 3, 4], [2], [1, 2]]]

    for graph in inputs:
        solution = Solution()
        result = solution.shortestPathLength(graph)
        print(result)


if __name__ == "__main__":
    main()
