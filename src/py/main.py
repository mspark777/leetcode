from __future__ import annotations
from typing import List, Optional
from collections import defaultdict


class Solution:
    def restoreArray(self, adjacentPairs: List[List[int]]) -> List[int]:
        graph = defaultdict[int, list[int]](list)

        for x, y in adjacentPairs:
            graph[x].append(y)
            graph[y].append(x)

        cur = self.root(graph)
        result = [cur]
        prev: Optional[int] = None

        while len(result) < len(graph):
            for neighbor in graph[cur]:
                if neighbor != prev:
                    result.append(neighbor)
                    prev = cur
                    cur = neighbor
                    break

        return result

    def root(self, graph: dict[int, list[int]]) -> int:
        for k, v in graph.items():
            if len(v) == 1:
                return k

        return -1


def main():
    inputs = ([[2, 1], [3, 4], [3, 2]], [[4, -2], [1, 4], [-3, 1]], [[100000, -100000]])

    for adjacentPairs in inputs:
        result = Solution().restoreArray(adjacentPairs)
        print(result)


if __name__ == "__main__":
    main()
