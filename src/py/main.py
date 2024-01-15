from __future__ import annotations
from typing import List
from collections import Counter


class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        winers = set([m[0] for m in matches])
        losers = Counter([m[1] for m in matches])

        filtered_winers = [w for w in winers if w not in losers]
        filtered_losers = [l for l, c in losers.items() if c == 1]

        filtered_winers.sort()
        filtered_losers.sort()

        return [filtered_winers, filtered_losers]


def main():
    input = (
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
    )

    for matches in input:
        result = Solution().findWinners(matches)
        print(result)


if __name__ == "__main__":
    main()
