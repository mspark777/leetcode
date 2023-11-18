from __future__ import annotations
from typing import List


class Solution:
    def maxFrequency(self, nums: List[int], k: int) -> int:
        nums.sort()
        left = 0
        result = 0
        curr = 0

        for right in range(len(nums)):
            target = nums[right]
            curr += target

            while (right - left + 1) * target - curr > k:
                curr -= nums[left]
                left += 1

            result = max(result, right - left + 1)

        return result


def main():
    inputs = (([1, 2, 4], 5), ([1, 4, 8, 13], 5))

    for nums, k in inputs:
        result = Solution().maxFrequency(nums, k)
        print(result)


if __name__ == "__main__":
    main()
