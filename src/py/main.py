from __future__ import annotations
from typing import List
from heapq import heapify, heappop, heappush
from math import ceil


class Solution:
    def maxKelements(self, nums: List[int], k: int) -> int:
        result = 0

        max_heap = [-i for i in nums]
        heapify(max_heap)

        while k > 0:
            k -= 1
            top = -heappop(max_heap)
            result += top
            heappush(max_heap, -ceil(top / 3))

        return result


def main():
    inputs = [([10, 10, 10, 10, 10], 5), ([1, 10, 3, 3, 3], 3)]

    for nums, k in inputs:
        result = Solution().maxKelements(nums, k)
        print(result)


if __name__ == "__main__":
    main()
