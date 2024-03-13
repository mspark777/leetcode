from __future__ import annotations
from math import sqrt


class Solution:
    def pivotInteger(self, n: int) -> int:
        sum = n * (n + 1) // 2
        pivot = int(sqrt(sum))
        return pivot if pivot * pivot == sum else -1


def main():
    input = [8, 1, 4]

    for n in input:
        result = Solution().pivotInteger(n)
        print(result)


if __name__ == "__main__":
    main()
