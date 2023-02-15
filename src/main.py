from __future__ import annotations
from typing import List


class Solution:
    def addToArrayForm(self, num: List[int], k: int) -> List[int]:
        result: list[int] = []
        cur = k

        for i in range(len(num) - 1, -1, -1):
            cur += num[i]
            result.append(cur % 10)
            cur //= 10

        while cur > 0:
            result.append(cur % 10)
            cur //= 10

        result.reverse()
        return result


def main():
    inputs: list[tuple[list[int], int]] = [
        ([1, 2, 0, 0], 34),
        ([2, 7, 4], 181),
        ([2, 1, 5], 806),
        ([1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3], 516),
        ([0], 10000),
    ]

    for num, k in inputs:
        solution = Solution()
        result = solution.addToArrayForm(num, k)
        print(result)


if __name__ == "__main__":
    main()
