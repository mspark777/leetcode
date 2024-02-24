from __future__ import annotations
from typing import List


class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        return max(nums[-1] * nums[-2] * nums[-3], nums[0] * nums[1] * nums[-1])


def main():
    input = ([1, 2, 3], [1, 2, 3, 4], [-1, -2, -3])

    for nums in input:
        result = Solution().maximumProduct(nums)
        print(result)


if __name__ == "__main__":
    main()
