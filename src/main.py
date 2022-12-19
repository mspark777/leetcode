from __future__ import annotations
from typing import List


class UnionFind:
    roots: list[int]
    ranks: list[int]

    def __init__(self, n: int):
        self.roots = [i for i in range(n)]
        self.ranks = [1 for _ in range(n)]

    def find(self, x: int) -> int:
        roots = self.roots
        if roots[x] != x:
            roots[x] = self.find(roots[x])

        return roots[x]

    def union(self, x: int, y: int):
        rootx = self.find(x)
        rooty = self.find(y)
        if rootx != rooty:
            ranks = self.ranks
            if ranks[rootx] > ranks[rooty]:
                temp = rootx
                rootx = rooty
                rooty = temp

            roots = self.roots
            roots[rootx] = rooty
            ranks[rooty] += ranks[rootx]


class Solution:
    def validPath(
        self, n: int, edges: List[List[int]], source: int, destination: int
    ) -> bool:
        uf = UnionFind(n)
        for [x, y] in edges:
            uf.union(x, y)

        return uf.find(source) == uf.find(destination)


class Input:
    n: int
    edges: list[list[int]]
    source: int
    destination: int

    def __init__(self, n: int, edges: list[list[int]], source: int, destination: int):
        self.n = n
        self.edges = edges
        self.source = source
        self.destination = destination


def main():
    inputs: list[Input] = [
        Input(3, [[0, 1], [1, 2], [2, 0]], 0, 2),
        Input(6, [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], 0, 5),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.validPath(
            input.n, input.edges, input.source, input.destination
        )
        print(result)


if __name__ == "__main__":
    main()
