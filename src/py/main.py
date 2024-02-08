from __future__ import annotations


class Solution:
    def numSquares(self, n: int) -> int:
        memo = [0xFFFFFFFF for _ in range(n + 1)]
        memo[0] = 0

        for i in range(1, n + 1):
            for j in range(1, int(i**0.5) + 1):
                memo[i] = min(memo[i], 1 + memo[i - (j * j)])

        return memo[n]


def main():
    input = (12, 13)

    for n in input:
        result = Solution().numSquares(n)
        print(result)


if __name__ == "__main__":
    main()
