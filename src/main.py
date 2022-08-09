"""
main
"""

from typing import Optional

class Solution:
    def numFactoredBinaryTrees(self, arr: list[int]) -> int:
        MOD = 10 ** 9 + 7
        LEN = len(arr)
        arr.sort()

        dp = [1] * LEN
        index = {k: v for v, k in enumerate(arr)}
        for i, parent in enumerate(arr):
            for j in range(i):
                left = arr[j]
                if (parent % left) == 0:
                    right = parent // left
                    if right in index:
                        dp[i] += dp[j] * dp[index[right]]
                        dp[i] %= MOD
        return sum(dp) % MOD

class Input:
    nums: list[int]
    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs: list[Input] = [
            Input([2, 4]),
            Input([2, 4, 5, 10]),
    ]

    s = Solution()
    for i in inputs:
        result = s.numFactoredBinaryTrees(i.nums)
        print(result)

if __name__ == "__main__":
    main()
