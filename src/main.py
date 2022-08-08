"""
main
"""

from typing import Optional
from bisect import bisect_left

class Solution:
    def lengthOfLIS(self, nums: list[int]) -> int:
        result = [nums[0]]
        for i in range(1, len(nums)):
            num = nums[i]
            if num > result[-1]:
                result.append(num)
            else:
                index =  bisect_left(result, num)
                result[index] = num
        return len(result)

class Input:
    nums: list[int]
    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs: list[Input] = [
            Input([10, 9, 2, 5, 3, 7, 101, 18]),
            Input([0, 1, 0, 3, 2, 3]),
            Input([7, 7, 7, 7, 7, 7, 7]),
    ]

    s = Solution()
    for i in inputs:
        result = s.lengthOfLIS(i.nums)
        print(result)

if __name__ == "__main__":
    main()
