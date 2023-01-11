from __future__ import annotations
from typing import List


class Solution:
    def minTime(self, n: int, edges: List[List[int]], hasApple: List[bool]) -> int:
        adj_mat = [[] for i in range(n)]
        for [l, r] in edges:
            adj_mat[l].append(r)
            adj_mat[r].append(l)

        return self.dfs(0, -1, adj_mat, hasApple)

    def dfs(
        self, node: int, prev: int, adj_mat: list[list[int]], has_apple: list[bool]
    ) -> int:
        total_time = 0
        child_time = 0

        for child in adj_mat[node]:
            if child == prev:
                continue

            child_time = self.dfs(child, node, adj_mat, has_apple)
            if (child_time > 0) or has_apple[child]:
                total_time += child_time + 2

        return total_time


class Input:
    n: int
    edges: list[list[int]]
    has_apple: list[bool]

    def __init__(self, n: int, edges: list[list[int]], has_apple: list[bool]):
        self.n = n
        self.edges = edges
        self.has_apple = has_apple


def main():
    inputs: list[Input] = [
        Input(
            7,
            [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            [False, False, True, False, True, True, False],
        ),
        Input(
            7,
            [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            [False, False, True, False, False, True, False],
        ),
        Input(
            7,
            [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            [False, False, False, False, False, False, False],
        ),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.minTime(input.n, input.edges, input.has_apple)
        print(result)


if __name__ == "__main__":
    main()
