from __future__ import annotations
from typing import List


class Solution:
    def maximumElementAfterDecrementingAndRearranging(self, arr: List[int]) -> int:
        n = len(arr)
        counts = [0] * (n + 1)

        for num in arr:
            counts[min(num, n)] += 1

        result = 1
        for num in range(2, n + 1):
            result = min(result + counts[num], num)

        return result


def main():
    inputs = ([2, 2, 1, 2, 1], [100, 1, 1000], [1, 2, 3, 4, 5])

    for arr in inputs:
        result = Solution().maximumElementAfterDecrementingAndRearranging(arr)
        print(result)


if __name__ == "__main__":
    main()
