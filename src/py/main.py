from typing import List

from utils import UnionFind


class Solution:
    def makeConnected(self, n: int, connections: List[List[int]]) -> int:
        if len(connections) < (n - 1):
            return -1

        uf = UnionFind(n)
        result = n

        for [a, b] in connections:
            if uf.find(a) != uf.find(b):
                result -= 1
                uf.union(a, b)

        return result - 1


def main():
    inputs: list[tuple[int, list[list[int]]]] = [
        (4, [[0, 1], [0, 2], [1, 2]]),
        (6, [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]),
        (6, [[0, 1], [0, 2], [0, 3], [1, 2]]),
    ]

    for n, connections in inputs:
        solution = Solution()
        result = solution.makeConnected(n, connections)
        print(result)


if __name__ == "__main__":
    main()
