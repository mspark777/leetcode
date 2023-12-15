from __future__ import annotations
from typing import List


class Solution:
    def calculateMinimumHP(self, dungeon: List[List[int]]) -> int:
        row_count = len(dungeon)
        col_count = len(dungeon[0])
        dp = [2**32 for _ in range(row_count + 1)]
        dp[row_count - 1] = 1
        for col in range(col_count - 1, -1, -1):
            for row in range(row_count - 1, -1, -1):
                health = min(dp[row], dp[row + 1]) - dungeon[row][col]
                dp[row] = max(health, 1)

        return dp[0]


def main():
    inputs = ([[-2, -3, 3], [-5, -10, 1], [10, 30, -5]], [[0]])

    for dungeon in inputs:
        result = Solution().calculateMinimumHP(dungeon)
        print(result)


if __name__ == "__main__":
    main()
