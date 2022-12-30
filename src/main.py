from __future__ import annotations
from typing import List


class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        results: list[list[int]] = []
        path: list[int] = []
        self.dfs(graph, results, path, 0)
        return results

    def dfs(
        self,
        graph: list[list[int]],
        results: list[list[int]],
        path: list[int],
        cur: int,
    ):
        path.append(cur)

        if cur == (len(graph) - 1):
            results.append(path[:])
        else:
            for next in graph[cur]:
                self.dfs(graph, results, path, next)

        path.pop()


def main():
    inputs: list[list[list[int]]] = [
        [[1, 2], [3], [3], []],
        [[4, 3, 1], [3, 2, 4], [3], [4], []],
    ]

    solution = Solution()
    for graph in inputs:
        result = solution.allPathsSourceTarget(graph)
        print(result)


if __name__ == "__main__":
    main()
