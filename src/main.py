from __future__ import annotations
from typing import Optional, List


class Solution:
    def minOperations(self, nums: List[int], x: int) -> int:
        target = sum(nums) - x
        n = len(nums)

        if target == 0:
            return n

        max_len = 0
        cur_sum = 0
        left = 0

        for right, val in enumerate(nums):
            cur_sum += val
            while (left <= right) and (cur_sum > target):
                cur_sum -= nums[left]
                left += 1

            if cur_sum == target:
                max_len = max(max_len, right - left + 1)

        return n - max_len if max_len != 0 else -1


def main():
    inputs = [([1, 1, 4, 2, 3], 5), ([5, 6, 7, 8, 9], 4), ([3, 2, 20, 1, 1, 3], 10)]

    for nums, x in inputs:
        solution = Solution()
        result = solution.minOperations(nums, x)
        print(result)


if __name__ == "__main__":
    main()
