from __future__ import annotations
from typing import List


class UnionFind:
    parent: list[int]
    ranks: list[int]

    def __init__(self, n: int):
        self.parent = list(range(n))
        self.ranks = [0] * n

    def find(self, u: int) -> int:
        if self.parent[u] != u:
            self.parent[u] = self.find(self.parent[u])
        return self.parent[u]

    def union(self, u: int, v: int):
        root_u = self.find(u)
        root_v = self.find(v)
        if root_u != root_v:
            if self.ranks[root_u] > self.ranks[root_v]:
                self.parent[root_v] = root_u
            elif self.ranks[root_u] < self.ranks[root_v]:
                self.parent[root_u] = root_v
            else:
                self.parent[root_v] = root_u
                self.ranks[root_u] += 1


class Solution:
    def is_land(self, grid: list[list[int]], x: int, y: int) -> bool:
        return grid[x][y] == 1

    def convert_to_index(self, x: int, y: int, total_cols: int) -> int:
        return x * total_cols + y

    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        total_rows = len(grid1)
        total_cols = len(grid1[0])
        uf = UnionFind(total_rows * total_cols)

        for x in range(total_rows):
            for y in range(total_cols):
                if self.is_land(grid2, x, y):
                    for direction in directions:
                        next_x = x + direction[0]
                        next_y = y + direction[1]
                        if (
                            0 <= next_x < total_rows
                            and 0 <= next_y < total_cols
                            and self.is_land(grid2, next_x, next_y)
                        ):
                            uf.union(
                                self.convert_to_index(x, y, total_cols),
                                self.convert_to_index(next_x, next_y, total_cols),
                            )
        is_sub_island = [True] * (total_rows * total_cols)
        for x in range(total_rows):
            for y in range(total_cols):
                if self.is_land(grid2, x, y) and not self.is_land(grid1, x, y):
                    root = uf.find(self.convert_to_index(x, y, total_cols))
                    is_sub_island[root] = False

        result = 0
        for x in range(total_rows):
            for y in range(total_cols):
                if self.is_land(grid2, x, y):
                    root = uf.find(self.convert_to_index(x, y, total_cols))
                    if is_sub_island[root]:
                        result += 1
                        is_sub_island[root] = False

        return result


def main():
    inputs = [
        (
            [
                [1, 1, 1, 0, 0],
                [0, 1, 1, 1, 1],
                [0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [1, 1, 0, 1, 1],
            ],
            [
                [1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1],
                [0, 1, 0, 0, 0],
                [1, 0, 1, 1, 0],
                [0, 1, 0, 1, 0],
            ],
        ),
        (
            [
                [1, 0, 1, 0, 1],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [1, 0, 1, 0, 1],
            ],
            [
                [0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [1, 0, 0, 0, 1],
            ],
        ),
    ]

    for grid1, grid2 in inputs:
        result = Solution().countSubIslands(grid1, grid2)
        print(result)


if __name__ == "__main__":
    main()
