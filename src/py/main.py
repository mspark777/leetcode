from __future__ import annotations
from math import sqrt


class Solution:
    def arrangeCoins(self, n: int) -> int:
        result = sqrt(2 * n + 0.25) - 0.5
        return int(result)


def main():
    input = (5, 8)

    for n in input:
        result = Solution().arrangeCoins(n)
        print(result)


if __name__ == "__main__":
    main()
