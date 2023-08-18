from __future__ import annotations
from typing import List
from collections import defaultdict


class Solution:
    def maximalNetworkRank(self, n: int, roads: List[List[int]]) -> int:
        adjacents: dict[int, set[int]] = defaultdict(set)
        for [a, b] in roads:
            adjacents[a].add(b)
            adjacents[b].add(a)

        result = 0
        for node0 in range(n):
            set0 = adjacents[node0]
            rank0 = len(set0)
            for node1 in range(node0 + 1, n):
                rank1 = len(adjacents[node1])
                rank = rank0 + rank1
                if node1 in set0:
                    rank -= 1

                result = max(result, rank)

        return result


def main():
    inputs = [
        (4, [[0, 1], [0, 3], [1, 2], [1, 3]]),
        (5, [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]]),
        (8, [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]]),
    ]

    for n, roads in inputs:
        solution = Solution()
        result = solution.maximalNetworkRank(n, roads)
        print(result)


if __name__ == "__main__":
    main()
