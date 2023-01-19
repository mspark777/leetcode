from __future__ import annotations
from typing import List


class Solution:
    def subarraysDivByK(self, nums: List[int], k: int) -> int:
        prefix = 0
        result = 0

        modGroups = [0 for i in range(k)]
        modGroups[0] = 1

        for num in nums:
            prefix = (prefix + k + (num % k)) % k
            result += modGroups[prefix]
            modGroups[prefix] += 1

        return result


def main():
    inputs: list[tuple[list[int], int]] = [([4, 5, 0, -2, -3, 1], 5), ([5], 9)]
    for nums, k in inputs:
        solution = Solution()
        result = solution.subarraysDivByK(nums, k)
        print(result)


if __name__ == "__main__":
    main()
