from __future__ import annotations
from typing import List


class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        left = 1
        right = num // 2
        while left <= right:
            mid = (left + right) // 2
            square = mid * mid
            if num < square:
                right = mid - 1
            elif num > square:
                left = mid + 1
            else:
                return True

        return num == 1


def main():
    inputs: list[int] = [16, 14, 1]

    for num in inputs:
        solution = Solution()
        result = solution.isPerfectSquare(num)
        print(result)


if __name__ == "__main__":
    main()
