from __future__ import annotations
from typing import Optional, List


class Solution:
    def countBits(self, n: int) -> List[int]:
        result = [0 for i in range(n + 1)]
        
        for i in range(1, n + 1):
            result[i] = self.solve(i, result)

        return result

    def solve(self, n: int, memo: list[int]) -> int:
        if n == 0:
            return 0
        elif n == 1:
            return 1
        elif memo[n] != 0:
            return memo[n]

        if (n & 1) == 1:
            memo[n] = self.solve(n // 2, memo) + 1
        else:
            memo[n] = self.solve(n // 2, memo)

        return memo[n]


def main():
    inputs: list[int] = [2, 5]

    solution = Solution()
    for n in inputs:
        result = solution.countBits(n)
        print(result)


if __name__ == "__main__":
    main()
