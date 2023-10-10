from __future__ import annotations
from typing import Optional, List


class Solution:
    def minOperations(self, nums: List[int]) -> int:
        n = len(nums)
        result = n
        new_nums = sorted(set(nums))
        j = 0

        for i in range(len(new_nums)):
            while (j < len(new_nums)) and (new_nums[j] < (new_nums[i] + n)):
                j += 1

            result = min(result, n - j + i)

        return result


def main():
    inputs = ([4, 2, 5, 3], [1, 2, 3, 5, 6], [1, 10, 100, 1000])

    for nums in inputs:
        result = Solution().minOperations(nums)
        print(result)


if __name__ == "__main__":
    main()
