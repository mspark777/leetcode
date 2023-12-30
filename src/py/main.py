from __future__ import annotations
from typing import List


class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        result = 0
        count = 0

        for num in nums:
            if num == 0:
                result = max(result, count)
                count = 0
            else:
                count += 1

        result = max(result, count)
        return result


def main():
    input = ([1, 1, 0, 1, 1, 1], [1, 0, 1, 1, 0, 1])

    for nums in input:
        result = Solution().findMaxConsecutiveOnes(nums)
        print(result)


if __name__ == "__main__":
    main()
