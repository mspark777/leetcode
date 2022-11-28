from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        lose_counts: Counter[int] = Counter()
        for [winner, loser] in matches:
            lose_counts[winner] = lose_counts.get(winner, 0)
            lose_counts[loser] = lose_counts.get(loser, 0) + 1

        winners: list[int] = []
        losers: list[int] = []

        for player, count in lose_counts.items():
            if count < 1:
                winners.append(player)
            elif count == 1:
                losers.append(player)

        winners.sort()
        losers.sort()

        return [winners, losers]


def main():
    inputs: list[list[list[int]]] = [
        [
            [1, 3],
            [2, 3],
            [3, 6],
            [5, 6],
            [5, 7],
            [4, 5],
            [4, 8],
            [4, 9],
            [10, 4],
            [10, 9],
        ],
        [[2, 3], [1, 3], [5, 4], [6, 4]],
    ]

    solution = Solution()
    for matches in inputs:
        result = solution.findWinners(matches)
        print(result)


if __name__ == "__main__":
    main()
