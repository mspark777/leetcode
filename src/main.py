"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def trap(self, height: list[int]) -> int:
        left = 0
        right = len(height) - 1
        left_max = 0
        right_max = 0
        result = 0

        while left < right:
            lheight = height[left]
            rheight = height[right]
            if lheight < rheight:
                left += 1
                if lheight >= left_max:
                    left_max = lheight
                else:
                    result += left_max - lheight
            else:
                right -= 1
                if rheight >= right_max:
                    right_max = rheight
                else:
                    result += right_max - rheight

        return result


class Input:
    height: list[int]

    def __init__(self, height: list[int]):
        self.height = height


def main():
    inputs: list[Input] = [
        Input([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
        Input([4, 2, 0, 3, 2, 5]),
    ]

    solution = Solution()
    for input in inputs:
        height = input.height
        result = solution.trap(height)
        print(result)


if __name__ == "__main__":
    main()
