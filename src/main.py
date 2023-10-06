from __future__ import annotations
from typing import Optional, List


class Solution:
    def integerBreak(self, n: int) -> int:
        if n <= 3:
            return n - 1

        if (n % 3) == 0:
            return 3 ** (n // 3)

        if (n % 3) == 1:
            return 3 ** ((n // 3) - 1) * 4

        return 3 ** (n // 3) * 2


def main():
    inputs = (2, 10)

    for n in inputs:
        result = Solution().integerBreak(n)
        print(result)


if __name__ == "__main__":
    main()
