from __future__ import annotations
from typing import List


class Solution:
    def peakIndexInMountainArray(self, arr: List[int]) -> int:
        left = 0
        right = len(arr) - 1
        while left < right:
            mid = (left + right) // 2
            if arr[mid] < arr[mid + 1]:
                left = mid + 1
            else:
                right = mid

        return left


def main():
    inputs = [[0, 1, 0], [0, 2, 1, 0], [0, 10, 5, 2]]

    for arr in inputs:
        solution = Solution()
        result = solution.peakIndexInMountainArray(arr)
        print(result)


if __name__ == "__main__":
    main()
