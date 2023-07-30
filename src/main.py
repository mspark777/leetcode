from __future__ import annotations


class Solution:
    def strangePrinter(self, s: str) -> int:
        n = len(s)
        dp = [[0 for _ in range(n)] for _ in range(n)]
        for l in range(1, n + 1):
            for left in range(0, n - l + 1):
                right = left + l - 1
                j = -1
                dp[left][right] = n

                for i in range(left, right):
                    if (s[i] != s[right]) and (j == -1):
                        j = i

                    if j != -1:
                        lmin = dp[left][right]
                        rmin = 1 + dp[j][i] + dp[i + 1][right]
                        dp[left][right] = min(lmin, rmin)

                if j == -1:
                    dp[left][right] = 0

        return dp[0][n - 1] + 1


def main():
    inputs = ["aaabbb", "aba"]

    for s in inputs:
        solution = Solution()
        result = solution.strangePrinter(s)
        print(result)


if __name__ == "__main__":
    main()
