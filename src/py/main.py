from __future__ import annotations
from typing import List


class Solution:
    def isMonotonic(self, nums: List[int]) -> bool:
        increasing = False
        decreasing = False

        for i in range(1, len(nums)):
            left = nums[i - 1]
            right = nums[i]
            if left < right:
                increasing = True
            elif left > right:
                decreasing = True

            if increasing and decreasing:
                return False

        return True


def main():
    input = ([1, 2, 2, 3], [6, 5, 4, 4], [1, 3, 2])

    for nums in input:
        result = Solution().isMonotonic(nums)
        print(result)


if __name__ == "__main__":
    main()
