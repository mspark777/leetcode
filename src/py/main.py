from __future__ import annotations
from typing import List


class Solution:
    def reductionOperations(self, nums: List[int]) -> int:
        nums.sort()
        result = 0
        up = 0

        for i in range(1, len(nums)):
            if nums[i] != nums[i - 1]:
                up += 1

            result += up

        return result


def main():
    inputs = (
        [5, 1, 3],
        [1, 1, 1],
        [1, 1, 2, 2, 3],
    )

    for nums in inputs:
        result = Solution().reductionOperations(nums)
        print(result)


if __name__ == "__main__":
    main()
