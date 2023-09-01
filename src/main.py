from __future__ import annotations
from typing import List


class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        RED = 0
        WHITE = 1
        low = 0
        mid = 0
        high = len(nums) - 1
        while mid <= high:
            num = nums[mid]
            if num == RED:
                nums[low], nums[mid] = nums[mid], nums[low]
                low += 1
                mid += 1
            elif num == WHITE:
                mid += 1
            else:
                nums[mid], nums[high] = nums[high], nums[mid]
                high -= 1


def main():
    inputs = [[2, 0, 2, 1, 1, 0], [2, 0, 1]]

    for nums in inputs:
        solution = Solution()
        solution.sortColors(nums)
        print(nums)


if __name__ == "__main__":
    main()
