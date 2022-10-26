from __future__ import annotations
from typing import Optional, List


class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        map = {0: 0}
        sum = 0

        for i in range(len(nums)):
            sum += nums[i]
            mod = sum % k
            if mod not in map:
                map[mod] = i + 1
            elif map[mod] < i:
                return True

        return False


def main():
    inputs: list[tuple[list[int], int]] = [
        ([23, 2, 4, 6, 7], 6),
        ([23, 2, 6, 4, 7], 6),
        ([23, 2, 6, 4, 7], 13),
    ]

    solution = Solution()
    for nums, k in inputs:
        result = solution.checkSubarraySum(nums, k)
        print(result)


if __name__ == "__main__":
    main()
