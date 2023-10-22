from __future__ import annotations
from typing import Optional, List


class Solution:
    def maximumScore(self, nums: List[int], k: int) -> int:
        n = len(nums)
        left = k
        right = k
        result = nums[k]
        curmin = nums[k]

        while left > 0 or right < (n - 1):
            l = nums[left - 1] if left > 0 else 0
            r = nums[right + 1] if right < n - 1 else 0
            if l < r:
                right += 1
                curmin = min(curmin, nums[right])
            else:
                left -= 1
                curmin = min(curmin, nums[left])

            result = max(result, curmin * (right - left + 1))

        return result


def main():
    inputs = (([1, 4, 3, 7, 4, 5], 3), ([5, 5, 4, 5, 4, 1, 1, 1], 0))

    for nums, k in inputs:
        result = Solution().maximumScore(nums, k)
        print(result)


if __name__ == "__main__":
    main()
