from __future__ import annotations
from typing import List

MAX: int = int(1e9)


class Graph:
    n: int
    matrix: list[list[int]]

    def __init__(self, n: int, edges: List[List[int]]):
        self.n = n
        self.matrix = [[MAX for _ in range(n)] for _ in range(n)]

        for f, to, cost in edges:
            self.matrix[f][to] = cost

        for i in range(n):
            self.matrix[i][i] = 0

        for i in range(n):
            for j in range(n):
                for k in range(n):
                    self.matrix[j][k] = min(
                        self.matrix[j][k], self.matrix[j][i] + self.matrix[i][k]
                    )

    def addEdge(self, edge: List[int]) -> None:
        f, to, cost = edge
        for i in range(self.n):
            for j in range(self.n):
                self.matrix[i][j] = min(
                    self.matrix[i][j], self.matrix[i][f] + self.matrix[to][j] + cost
                )

    def shortestPath(self, node1: int, node2: int) -> int:
        cost = self.matrix[node1][node2]
        return cost if cost != MAX else -1


def main():
    g = Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]])
    print(g.shortestPath(3, 2))
    print(g.shortestPath(0, 3))
    g.addEdge([1, 3, 4])
    print(g.shortestPath(0, 3))


if __name__ == "__main__":
    main()
