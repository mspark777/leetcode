from __future__ import annotations
from typing import Optional, List


class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        first = self.search(nums, -1, 0, len(nums) - 1, target, True)
        second = self.search(nums, -1, 0, len(nums) - 1, target, False)
        return [first, second]

    def search(
        self,
        nums: list[int],
        result: int,
        left: int,
        right: int,
        target: int,
        first: bool,
    ) -> int:
        if left > right:
            return result

        mid = (left + right) // 2
        num = nums[mid]
        if num > target:
            return self.search(nums, result, left, mid - 1, target, first)
        elif num < target:
            return self.search(nums, result, mid + 1, right, target, first)
        elif first:
            return self.search(nums, mid, left, mid - 1, target, first)

        return self.search(nums, mid, mid + 1, right, target, first)


def main():
    inputs = (([5, 7, 7, 8, 8, 10], 8), ([5, 7, 7, 8, 8, 10], 6), ([], 0))

    for nums, target in inputs:
        result = Solution().searchRange(nums, target)
        print(result)


if __name__ == "__main__":
    main()
