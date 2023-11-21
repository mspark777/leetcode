from __future__ import annotations
from collections import defaultdict
from typing import List


class Solution:
    def countNicePairs(self, nums: List[int]) -> int:
        arr = [n - self.rev(n) for n in nums]

        dic = defaultdict[int, int](int)
        result = 0
        MOD = 10**9 + 7

        for num in arr:
            result = (result + dic[num]) % MOD
            dic[num] += 1

        return result

    def rev(self, num: int) -> int:
        result = 0
        while num > 0:
            result = result * 10 + num % 10
            num //= 10

        return result


def main():
    inputs = ([42, 11, 1, 97], [13, 10, 35, 24, 76])

    for nums in inputs:
        result = Solution().countNicePairs(nums)
        print(result)


if __name__ == "__main__":
    main()
