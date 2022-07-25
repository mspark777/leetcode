"""
solution
"""
from __future__ import annotations

class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        first = self.search(nums, target, True)
        last = self.search(nums, target, False)
        return [first, last]

    def search(self, nums: list[int], target: int, first: bool) -> int:
        result = -1
        left = 0
        right = len(nums) - 1

        while left <= right:
            mid = (left + right) // 2
            num = nums[mid]

            if num > target:
                right = mid - 1
            elif num < target:
                left = mid + 1
            else:
                result = mid
                if first:
                    right = mid - 1
                else:
                    left = mid + 1

        return result
