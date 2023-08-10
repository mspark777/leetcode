from __future__ import annotations
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> bool:
        if not nums:
            return False

        left = 0
        right = len(nums)

        while left < right:
            mid = (left + right) // 2
            mnum = nums[mid]
            if mnum == target:
                return True

            lnum = nums[left]
            if lnum == mnum:
                left += 1
                continue

            pivot_array = lnum <= mnum
            target_array = lnum <= target
            if pivot_array != target_array:
                if pivot_array:
                    left = mid + 1
                else:
                    right = mid
            else:
                if mnum < target:
                    left = mid + 1
                else:
                    right = mid

        return False


def main():
    inputs = [
        ([2, 5, 6, 0, 0, 1, 2], 0),
        ([2, 5, 6, 0, 0, 1, 2], 3),
        ([1, 0, 1, 1, 1], 0),
    ]

    for nums, target in inputs:
        solution = Solution()
        result = solution.search(nums, target)
        print(result)


if __name__ == "__main__":
    main()
