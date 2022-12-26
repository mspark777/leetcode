from __future__ import annotations
from typing import List


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        last = len(nums) - 1
        for i in range(last - 1, -1, -1):
            cur = i + nums[i]
            if cur >= last:
                last = i

        return last < 1


def main():
    inputs: list[list[int]] = [[2, 3, 1, 1, 4], [3, 2, 1, 0, 4]]

    solution = Solution()
    for nums in inputs:
        result = solution.canJump(nums)
        print(result)


if __name__ == "__main__":
    main()
