from __future__ import annotations
from typing import List


class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        result: list[str] = []
        for i, num in enumerate(nums):
            cur = num[i]
            result.append("1" if cur == "0" else "0")

        return "".join(result)


def main():
    inputs = (["01", "10"], ["00", "01"], ["111", "011", "001"])

    for nums in inputs:
        result = Solution().findDifferentBinaryString(nums)
        print(result)


if __name__ == "__main__":
    main()
