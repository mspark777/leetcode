from __future__ import annotations
from typing import List


class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        l = len(nums)
        k %= l
        pivot = l - k
        left = nums[pivot:]
        right = nums[:pivot]

        i = 0
        for n in left:
            nums[i] = n
            i += 1

        for n in right:
            nums[i] = n
            i += 1


def main():
    inputs = (([1, 2, 3, 4, 5, 6, 7], 3), ([-1, -100, 3, 99], 2))

    for nums, k in inputs:
        Solution().rotate(nums, k)
        print(nums)


if __name__ == "__main__":
    main()
