from __future__ import annotations
from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        n = len(nums)
        l = 0
        r = n - 1
        i = n - 1
        result = [0] * n
        while i >= 0:
            left = nums[l] * nums[l]
            right = nums[r] * nums[r]
            if left > right:
                result[i] = left
                l += 1
            else:
                result[i] = right
                r -= 1
            i -= 1

        return result


def main():
    input = [[-4, -1, 0, 3, 10], [-7, -3, 2, 3, 11]]
    for nums in input:
        result = Solution().sortedSquares(nums)
        print(result)


if __name__ == "__main__":
    main()
