from __future__ import annotations
from typing import Optional, List


class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        if len(nums) < 3:
            return False

        min = 2147483647
        middle = min

        for n in nums:
            if n <= min:
                min = n
            elif n <= middle:
                middle = n
            else:
                return True

        return False


def main():
    inputs: list[list[int]] = [
        [1, 2, 3, 4, 5],
        [5, 4, 3, 2, 1],
        [2, 1, 5, 0, 4, 6],
        [2, 6, 1, 8],
    ]

    solution = Solution()
    for nums in inputs:
        result = solution.increasingTriplet(nums)
        print(result)


if __name__ == "__main__":
    main()
