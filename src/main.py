from __future__ import annotations
from typing import List
from queue import PriorityQueue


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        queue: PriorityQueue[int] = PriorityQueue(k + 1)
        for num in nums:
            queue.put(num)
            if queue.qsize() > k:
                queue.get()

        result = queue.get()
        return result


def main():
    inputs = [([3, 2, 1, 5, 6, 4], 2), ([3, 2, 3, 1, 2, 4, 5, 5, 6], 4)]

    for nums, k in inputs:
        solution = Solution()
        result = solution.findKthLargest(nums, k)
        print(result)


if __name__ == "__main__":
    main()
