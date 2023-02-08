from __future__ import annotations
from typing import List


class Solution:
    def jump(self, nums: List[int]) -> int:
        result = 0
        curend = 0
        curfar = 0

        for i in range(len(nums) - 1):
            curfar = max(curfar, i + nums[i])

            if i == curend:
                result += 1
                curend = curfar

        return result


def main():
    inputs: list[list[int]] = [[2, 3, 1, 1, 4], [2, 3, 0, 1, 4]]

    for nums in inputs:
        solution = Solution()
        result = solution.jump(nums)
        print(result)


if __name__ == "__main__":
    main()
