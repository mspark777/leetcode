from __future__ import annotations
from typing import Optional, List


class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        nums_len = len(nums)
        temps = [0] * nums_len
        for num in nums:
            temps[num - 1] += 1

        missing = -1
        dup = -1

        for i in range(nums_len):
            temp = temps[i]
            if temp <= 0:
                missing = i + 1
            elif temp > 1:
                dup = i + 1

            if (missing >= 0) and (dup >= 0):
                break

        return [dup, missing]


def main():
    inputs: list[list[int]] = [[1, 2, 2, 4], [1, 1]]

    solution = Solution()
    for nums in inputs:
        result = solution.findErrorNums(nums)
        print(result)


if __name__ == "__main__":
    main()
