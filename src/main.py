from __future__ import annotations
from typing import Optional, List


class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        slen = len(s)
        tlen = len(t)

        dp = [[0 if i != 0 else 1 for i in range(tlen + 1)] for _ in range(slen + 1)]

        for i in range(1, slen + 1):
            for j in range(1, tlen + 1):
                if s[i - 1] == t[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j]
                else:
                    dp[i][j] = dp[i - 1][j]

        return dp[slen][tlen]


def main():
    inputs = [("rabbbit", "rabbit"), ("babgbag", "bag")]

    for s, t in inputs:
        solution = Solution()
        result = solution.numDistinct(s, t)
        print(result)


if __name__ == "__main__":
    main()
