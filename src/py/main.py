from __future__ import annotations
from typing import List


class Solution:
    def arrayPairSum(self, nums: List[int]) -> int:
        nums.sort()
        return sum([n for i, n in enumerate(nums) if i % 2 == 0])


def main():
    input = ([1, 4, 3, 2], [6, 2, 6, 5, 1, 2])

    for nums in input:
        result = Solution().arrayPairSum(nums)
        print(result)


if __name__ == "__main__":
    main()
