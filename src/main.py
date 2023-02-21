from __future__ import annotations
from typing import List


class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        left = 0
        right = len(nums) - 1

        while left < right:
            middle = (left + right) // 2

            if (middle & 1) == 1:
                if nums[middle] != nums[middle + 1]:
                    left = middle + 1
                else:
                    right = middle
            else:
                if nums[middle] == nums[middle + 1]:
                    left = middle + 1
                else:
                    right = middle

        return nums[left]


def main():
    inputs: list[list[int]] = [[1, 1, 2, 3, 3, 4, 4, 8, 8], [3, 3, 7, 7, 10, 11, 11]]

    for nums in inputs:
        solution = Solution()
        result = solution.singleNonDuplicate(nums)
        print(result)


if __name__ == "__main__":
    main()
