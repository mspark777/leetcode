from __future__ import annotations
from typing import List


class Solution:
    def minimizeMax(self, nums: List[int], p: int) -> int:
        nums.sort()

        left = 0
        right = nums[-1] - nums[0]
        while left < right:
            mid = (left + right) // 2
            if self.count_valid_pairs(nums, mid) >= p:
                right = mid
            else:
                left = mid + 1

        return left

    def count_valid_pairs(self, nums: list[int], threshold: int) -> int:
        index = 1
        count = 0
        while index < len(nums):
            first = nums[index - 1]
            second = nums[index]
            diff = second - first
            if diff <= threshold:
                count += 1
                index += 1

            index += 1

        return count


def main():
    inputs = [([10, 1, 2, 7, 1, 3], 2), ([4, 2, 1, 2], 1)]

    for nums, p in inputs:
        solution = Solution()
        result = solution.minimizeMax(nums, p)
        print(result)


if __name__ == "__main__":
    main()
