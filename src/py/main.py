from __future__ import annotations
from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        i_num = 0
        j_num = 0
        for num in nums:
            if num > i_num:
                j_num = i_num
                i_num = num
            else:
                j_num = max(j_num, num)

        return (i_num - 1) * (j_num - 1)


def main():
    inputs = (
        [3, 4, 5, 2],
        [1, 5, 4, 5],
        [3, 7],
        [2, 2, 1, 8, 1, 5, 4, 5, 2, 10, 3, 6, 5, 2, 3],
    )

    for nums in inputs:
        result = Solution().maxProduct(nums)
        print(result)


if __name__ == "__main__":
    main()
