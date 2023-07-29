from __future__ import annotations
from math import ceil


class Solution:
    def soupServings(self, n: int) -> float:
        dp: dict[int, dict[int, float]] = {}
        dp[0] = {0: 0.5}

        m = ceil(n / 25)

        for k in range(1, m + 1):
            dp[k] = {}
            dp[0][k] = 1
            dp[k][0] = 0

            for j in range(1, k + 1):
                dp[j][k] = self.calculate_dp(j, k, dp)
                dp[k][j] = self.calculate_dp(k, j, dp)

            if dp[k][k] > (1 - 1e-5):
                return 1

        return dp[m][m]

    def calculate_dp(self, i: int, j: int, dp: dict[int, dict[int, float]]) -> float:
        dp0 = dp[max(0, i - 4)][j]
        dp1 = dp[max(0, i - 3)][j - 1]
        dp2 = dp[max(0, i - 2)][max(0, j - 2)]
        dp3 = dp[i - 1][max(0, j - 3)]
        sum = dp0 + dp1 + dp2 + dp3
        return sum / 4


def main():
    inputs = [50, 100]

    for n in inputs:
        solution = Solution()
        result = solution.soupServings(n)
        print(result)


if __name__ == "__main__":
    main()
