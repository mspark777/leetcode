from __future__ import annotations
from typing import List, Callable


class Solution:
    def findSpecialInteger(self, arr: List[int]) -> int:
        n = len(arr)
        candidates = (arr[n // 4], arr[n // 2], arr[(3 * n) // 4])
        target = n // 4

        for candidate in candidates:
            left = self.search(arr, candidate, self.lower_bound)
            right = self.search(arr, candidate, self.upper_bound)
            l = right - left
            if l > target:
                return candidate

        return -1

    def search(
        self, arr: list[int], target: int, check: Callable[[int, int], bool]
    ) -> int:
        left = 0
        right = len(arr)
        while left < right:
            mid = (left + right) // 2
            if check(arr[mid], target):
                right = mid
            else:
                left = mid + 1

        return left

    def upper_bound(self, mid: int, target: int) -> bool:
        return mid > target

    def lower_bound(self, mid: int, target: int) -> bool:
        return mid >= target


def main():
    inputs = ([1, 2, 2, 6, 6, 6, 6, 7, 10], [1, 1])

    for arr in inputs:
        result = Solution().findSpecialInteger(arr)
        print(result)


if __name__ == "__main__":
    main()
