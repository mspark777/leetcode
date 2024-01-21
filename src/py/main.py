from __future__ import annotations
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        rob = 0
        norob = 0

        for num in nums:
            gorob = norob + num
            norob = max(norob, rob)
            rob = gorob

        return max(rob, norob)


def main():
    input = ([1, 2, 3, 1], [2, 7, 9, 3, 1])

    for nums in input:
        result = Solution().rob(nums)
        print(result)


if __name__ == "__main__":
    main()
