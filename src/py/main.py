from __future__ import annotations
from typing import List


class Solution:
    def maxSumAfterPartitioning(self, arr: List[int], k: int) -> int:
        n = len(arr)

        dp = [0 for _ in range(n + 1)]

        for start in range(n - 1, -1, -1):
            curr_max = 0
            end = min(n, start + k)

            for i in range(start, end):
                curr_max = max(curr_max, arr[i])
                dp[start] = max(dp[start], dp[i + 1] + curr_max * (i - start + 1))

        return dp[0]


def main():
    input = (
        ([1, 15, 7, 9, 2, 5, 10], 3),
        ([1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
        ([1], 1),
    )

    for arr, k in input:
        result = Solution().maxSumAfterPartitioning(arr, k)
        print(result)


if __name__ == "__main__":
    main()
