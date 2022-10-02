from __future__ import annotations
from typing import Optional, List


class Solution:
    def numRollsToTarget(self, n: int, k: int, target: int) -> int:
        MOD = 1000000007

        dp = [0] * (target + 1)
        dp[0] = 1

        for i in range(1, n + 1):
            for j in range(target, -1, -1):
                dp[j] = 0

                for p in range(1, k + 1):
                    if j >= p:
                        dp[j] = (dp[j] + dp[j - p]) % MOD
                    else:
                        break

        return dp[target]


class Input:
    n: int
    k: int
    target: int

    def __init__(self, n: int, k: int, target: int) -> None:
        self.n = n
        self.k = k
        self.target = target


def main():
    inputs: list[Input] = [Input(1, 6, 3), Input(2, 6, 7), Input(30, 30, 500)]

    solution = Solution()
    for input in inputs:
        n = input.n
        k = input.k
        target = input.target
        result = solution.numRollsToTarget(n, k, target)
        print(result)


if __name__ == "__main__":
    main()
