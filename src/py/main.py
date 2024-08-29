from __future__ import annotations
from typing import List


class UnionFind:
    unimap: dict[int, int]
    islands: int

    def __init__(self) -> None:
        self.unimap = {}
        self.islands = 0

    def find(self, x: int) -> int:
        if x not in self.unimap:
            self.unimap[x] = x
            self.islands += 1

        p = self.unimap[x]
        if x != p:
            self.unimap[x] = self.find(p)

        return self.unimap[x]

    def union(self, x: int, y: int) -> None:
        x = self.find(x)
        y = self.find(y)
        if x != y:
            self.unimap[x] = y
            self.islands -= 1


class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        uf = UnionFind()
        for stone in stones:
            uf.union(stone[0], ~stone[1])

        return len(stones) - uf.islands


def main():
    inputs = [
        [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]],
        [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]],
        [[0, 0]],
    ]

    for stones in inputs:
        result = Solution().removeStones(stones)
        print(result)


if __name__ == "__main__":
    main()
