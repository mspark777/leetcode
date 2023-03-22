from typing import List

from utils import UnionFind


class Solution:
    def minScore(self, n: int, roads: List[List[int]]) -> int:
        uf = UnionFind(n + 1)
        result = 0xFFFFFFFF

        for road in roads:
            uf.union(road[0], road[1])

        for road in roads:
            if uf.find(1) == uf.find(road[0]):
                result = min(result, road[2])

        return result


def main():
    inputs: list[tuple[int, list[list[int]]]] = [
        (4, [[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]]),
        (4, [[1, 2, 2], [1, 3, 4], [3, 4, 7]]),
    ]

    for n, roads in inputs:
        solution = Solution()
        result = solution.minScore(n, roads)
        print(result)


if __name__ == "__main__":
    main()
