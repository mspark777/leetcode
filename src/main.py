from __future__ import annotations
from typing import List


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if target < nums[0]:
            return 0
        elif target > nums[-1]:
            return len(nums)

        begin = 0
        end = len(nums)
        while begin < end:
            middle = (begin + end) // 2
            num = nums[middle]
            if num < target:
                begin = middle + 1
            elif num > target:
                end = middle
            else:
                return middle

        return begin


def main():
    inputs: list[tuple[list[int], int]] = [
        ([1, 3, 5, 6], 5),
        ([1, 3, 5, 6], 2),
        ([1, 3, 5, 6], 7),
    ]

    for nums, target in inputs:
        solution = Solution()
        result = solution.searchInsert(nums, target)
        print(result)


if __name__ == "__main__":
    main()
