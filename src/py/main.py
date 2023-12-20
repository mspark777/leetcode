from __future__ import annotations
from typing import List


class Solution:
    def maxWidthRamp(self, nums: List[int]) -> int:
        stack: list[int] = [0]
        n = len(nums)

        for i in range(1, n):
            top = stack[-1]
            if nums[top] > nums[i]:
                stack.append(i)

        result = 0
        for i in range(n - 1, -1, -1):
            if not stack:
                break
            elif i < result:
                break

            while stack and nums[stack[-1]] <= nums[i]:
                result = max(result, i - stack.pop())

        return result


def main():
    input = ([6, 0, 8, 2, 1, 5], [9, 8, 1, 0, 1, 9, 4, 0, 4, 1], [2, 2, 1])

    for nums in input:
        result = Solution().maxWidthRamp(nums)
        print(result)


if __name__ == "__main__":
    main()
