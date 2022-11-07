from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Solution:
    def maximum69Number (self, num: int) -> int:
        nums = list(str(num))
        for i, ch in enumerate(nums):
            if ch == '6':
                nums[i] = '9'
                break

        return int("".join(nums))


def main():
    inputs: list[int] = [9669, 9996, 9999]

    solution = Solution()
    for num in inputs:
        result = solution.maximum69Number(num)
        print(result)


if __name__ == "__main__":
    main()
