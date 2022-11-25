from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def sumSubarrayMins(self, arr: List[int]) -> int:
        stack: list[int] = []
        dp = [0] * len(arr)

        for i, n in enumerate(arr):
            while stack and (arr[stack[-1]] >= n):
                stack.pop()

            if stack:
                top = stack[-1]
                dp[i] = dp[top] + (i - top) * n
            else:
                dp[i] = (i + 1) * n

            stack.append(i)

        result = 0
        for count in dp:
            result += count
            result %= 1000000007

        return result


def main():
    inputs: list[list[int]] = [[3, 1, 2, 4], [11, 81, 94, 43, 3]]

    solution = Solution()
    for arr in inputs:
        result = solution.sumSubarrayMins(arr)
        print(result)


if __name__ == "__main__":
    main()
