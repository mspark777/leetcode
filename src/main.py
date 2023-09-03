from __future__ import annotations


class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [[1 if (r == 0) or (c == 0) else 0 for c in range(n)] for r in range(m)]

        for r in range(1, m):
            for c in range(1, n):
                dp[r][c] = dp[r - 1][c] + dp[r][c - 1]

        return dp[-1][-1]


def main():
    inputs = [(3, 7), (3, 2)]

    for m, n in inputs:
        solution = Solution()
        result = solution.uniquePaths(m, n)
        print(result)


if __name__ == "__main__":
    main()
