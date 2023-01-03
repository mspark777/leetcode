from __future__ import annotations
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        left = 0
        right = len(nums) - 1
        while left <= right:
            mid = (left + right) // 2
            if nums[mid] == target:
                return mid

            if nums[mid] >= nums[left]:
                if (target >= nums[left]) and (target < nums[mid]):
                    right = mid - 1
                else:
                    left = mid + 1
            else:
                if (target > nums[mid]) and (target <= nums[right]):
                    left = mid + 1
                else:
                    right = mid - 1

        return -1


def main():
    inputs: list[tuple[list[int], int]] = [
        ([4, 5, 6, 7, 0, 1, 2], 0),
        ([4, 5, 6, 7, 0, 1, 2], 3),
        ([1], 0),
    ]

    solution = Solution()
    for nums, target in inputs:
        result = solution.search(nums, target)
        print(result)


if __name__ == "__main__":
    main()
