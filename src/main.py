from __future__ import annotations
from typing import Optional, List


class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        left = 0
        right = len(arr) - k

        while left < right:
            mid = (left + right) // 2
            a = arr[mid + k] - x
            b = x - arr[mid]
            if a < b:
                left = mid + 1
            else:
                right = mid

        return arr[left : left + k]


class Input:
    arr: List[int]
    k: int
    x: int

    def __init__(self, arr: List[int], k: int, x: int) -> None:
        self.arr = arr
        self.k = k
        self.x = x


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 4, 5], 4, 3),
        Input([1, 2, 3, 4, 5], 4, -1),
    ]

    solution = Solution()
    for input in inputs:
        arr = input.arr
        k = input.k
        x = input.x
        result = solution.findClosestElements(arr, k, x)
        print(result)


if __name__ == "__main__":
    main()
