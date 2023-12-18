from __future__ import annotations
from typing import List


class Compare(str):
    def __lt__(self, right: str) -> bool:
        return self + right > right + self


class Solution:
    def largestNumber(self, nums: List[int]) -> str:
        strs = sorted(map(str, nums), key=Compare)
        return "".join(strs) if strs[0] != "0" else "0"


def main():
    inputs = ([10, 2], [3, 30, 34, 5, 9])

    for nums in inputs:
        result = Solution().largestNumber(nums)
        print(result)


if __name__ == "__main__":
    main()
