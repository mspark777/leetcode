from typing import List
from collections import Counter


class Solution:
    def unequalTriplets(self, nums: List[int]) -> int:
        result = 0
        counts = Counter(nums)
        left = 0
        right = len(nums)
        for count in counts.values():
            right -= count
            result += left * count * right
            left += count


        return result
        


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs = [
        Input([4,4,2,4,3]),
        Input([1,1,1,1,1]),
    ]

    for input in inputs:
        result = Solution().unequalTriplets(input.nums)
        print(result)


if __name__ == "__main__":
    main()
