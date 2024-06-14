from __future__ import annotations
from typing import List


class Solution:
    def minIncrementForUnique(self, nums: List[int]) -> int:
        result = 0
        nums.sort()

        for i in range(1, len(nums)):
            prev = nums[i - 1]
            curr = nums[i]
            if prev >= curr:
                result += prev + 1 - curr
                nums[i] = prev + 1

        return result


def main():
    input: list[list[int]] = [[1, 2, 2], [3, 2, 1, 2, 1, 7]]

    for nums in input:
        result = Solution().minIncrementForUnique(nums)
        print(result)


if __name__ == "__main__":
    main()
