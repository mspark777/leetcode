from __future__ import annotations
from typing import List


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        stack: list[int] = []
        result = 0

        for i, h in enumerate(heights + [0]):
            while stack and (heights[stack[-1]] >= h):
                idx = stack.pop()
                w = i if not stack else i - stack[-1] - 1
                result = max(result, heights[idx] * w)
            stack.append(i)

        return result


def main():
    inputs = [[2, 1, 5, 6, 2, 3], [2, 4]]

    for heights in inputs:
        solution = Solution()
        result = solution.largestRectangleArea(heights)
        print(result)


if __name__ == "__main__":
    main()
