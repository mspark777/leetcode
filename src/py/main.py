from __future__ import annotations
from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        num_count = len(nums)
        result = [1] * num_count
        for i in range(0, num_count - 1):
            result[i + 1] = result[i] * nums[i]

        right = nums[-1]
        for i in range(num_count - 2, -1, -1):
            result[i] *= right
            right *= nums[i]

        return result


def main():
    inputs = ([1, 2, 3, 4], [-1, 1, 0, -3, 3])

    for nums in inputs:
        result = Solution().productExceptSelf(nums)
        print(result)


if __name__ == "__main__":
    main()
