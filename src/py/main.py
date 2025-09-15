from typing import List
from collections import defaultdict


class Solution:
    def countQuadruplets(self, nums: List[int]) -> int:
        result = 0
        n = len(nums)
        seen = defaultdict(int)
        for a in range(n - 1, 0, -1):
            for b in range(a - 1, -1, -1):
                result += seen[nums[a] + nums[b]]
            for d in range(n - 1, a, -1):
                seen[nums[d] - nums[a]] += 1

        return result


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs = [
        Input([1, 2, 3, 6]),
        Input([3, 3, 6, 4, 5]),
        Input([1, 1, 1, 3, 5]),
    ]

    for input in inputs:
        result = Solution().countQuadruplets(input.nums)
        print(result)


if __name__ == "__main__":
    main()
