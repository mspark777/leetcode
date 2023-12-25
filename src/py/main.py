from __future__ import annotations


class Solution:
    def numOfArrays(self, n: int, m: int, k: int) -> int:
        MOD = 10**9 + 7
        dp = [[[0] * (k + 1) for _ in range(m + 1)] for _ in range(n + 1)]
        dp[0][0][0] = 1

        for i in range(1, n + 1):
            for j in range(1, m + 1):
                for cost in range(1, k + 1):
                    dp[i][j][cost] = (dp[i][j][cost] + dp[i - 1][j][cost] * j) % MOD
                    for prev in range(j):
                        dp[i][j][cost] = (
                            dp[i][j][cost] + dp[i - 1][prev][cost - 1]
                        ) % MOD
        total_ways = sum(dp[n][j][k] for j in range(1, m + 1)) % MOD

        return total_ways


def main():
    input = ((2, 3, 1), (5, 2, 3), (9, 1, 1))

    for n, m, k in input:
        result = Solution().numOfArrays(n, m, k)
        print(result)


if __name__ == "__main__":
    main()
