from __future__ import annotations
from typing import List


class Solution:
    def find(self, nums: list[int], val: int) -> int:
        left = 0
        right = len(nums) - 1
        index = right + 1
        while left <= right:
            mid = (left + right) // 2

            if nums[mid] >= val:
                index = mid
                right = mid - 1
            else:
                left = mid + 1

        return index

    def specialArray(self, nums: List[int]) -> int:
        nums.sort()

        n = len(nums)
        for i in range(1, n + 1):
            k = self.find(nums, i)

            if (n - k) == i:
                return i

        return -1


def main():
    input: list[list[int]] = [[3, 5], [0, 0], [0, 4, 3, 0, 4]]

    for nums in input:
        result = Solution().specialArray(nums)
        print(result)


if __name__ == "__main__":
    main()
