from __future__ import annotations
from typing import List


class DFS:
    N: int
    result: list[int]
    counts: list[int]
    graph: list[set[int]]

    def __init__(self, N: int):
        self.N = N
        self.graph = []
        self.counts = [1 for _ in range(N)]
        self.result = [0 for _ in range(N)]

    def dfs(self, node: int, parent: int):
        result = self.result
        counts = self.counts
        graph = self.graph
        for child in graph[node]:
            if child != parent:
                self.dfs(child, node)
                counts[node] += counts[child]
                result[node] += result[child] + counts[child]

    def dfs2(self, node: int, parent: int):
        result = self.result
        counts = self.counts
        N = self.N
        graph = self.graph
        for child in graph[node]:
            if child != parent:
                result[child] = result[node] - counts[child] + N - counts[child]
                self.dfs2(child, node)

    def sum_of_distances_in_tree(self, edges: list[list[int]]) -> list[int]:
        for i in range(self.N):
            self.graph.append(set())

        for edge in edges:
            self.graph[edge[0]].add(edge[1])
            self.graph[edge[1]].add(edge[0])

        self.dfs(0, -1)
        self.dfs2(0, -1)
        return self.result


class Solution:
    def sumOfDistancesInTree(self, n: int, edges: List[List[int]]) -> List[int]:
        dfs = DFS(n)
        return dfs.sum_of_distances_in_tree(edges)


class Input:
    n: int
    edges: list[list[int]]

    def __init__(self, n: int, edges: list[list[int]]):
        self.n = n
        self.edges = edges


def main():
    inputs: list[Input] = [
        Input(6, [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]),
        Input(1, []),
        Input(2, [[1, 0]]),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.sumOfDistancesInTree(input.n, input.edges)
        print(result)


if __name__ == "__main__":
    main()
