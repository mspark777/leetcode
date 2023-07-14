from __future__ import annotations
from typing import List


class Solution:
    def longestSubsequence(self, arr: List[int], difference: int) -> int:
        dp: dict[int, int] = {}
        result = 1

        for num in arr:
            before = dp.get(num - difference)
            if before is None:
                before = 0
            now = before + 1
            dp[num] = now

            result = max(result, now)

        return result


def main():
    inputs = [([1, 2, 3, 4], 1), ([1, 3, 5, 7], 1), ([1, 5, 7, 8, 5, 3, 4, 2, 1], -2)]

    for arr, difference in inputs:
        solution = Solution()
        result = solution.longestSubsequence(arr, difference)
        print(result)


if __name__ == "__main__":
    main()
