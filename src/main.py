"""
main
"""

from typing import Optional, SupportsComplex

class Solution:
    def combinationSum4(self, nums: list[int], target: int) -> int:
        result = [0 for n in range(target + 1)]
        result[0] = 1

        for i in range(1, target + 1):
            result[i] = sum(result[i - num] for num in nums if i >= num)

        return result[-1]




class Input:
    nums: list[int]
    target: int
    def __init__(self, nums: list[int], target: int):
        self.nums = nums
        self.target = target

def main():
    inputs: list[Input] = [
            Input([1, 2, 3], 4),
            Input([9], 3),
    ]

    s = Solution()
    for i in inputs:
        result = s.combinationSum4(i.nums, i.target)
        print(result)




if __name__ == "__main__":
    main()
