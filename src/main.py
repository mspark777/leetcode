from __future__ import annotations
from typing import List


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        j = 1
        for i in range(2, len(nums)):
            if nums[i] != nums[j - 1]:
                j += 1
                nums[j] = nums[i]

        return j + 1


def main():
    inputs = [[1, 1, 1, 2, 2, 3], [0, 0, 1, 1, 1, 1, 2, 3, 3]]

    for nums in inputs:
        solution = Solution()
        result = solution.removeDuplicates(nums)
        print(result)


if __name__ == "__main__":
    main()
