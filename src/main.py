from __future__ import annotations
from typing import List
from collections import deque


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        queue: deque[int] = deque()
        result: list[int] = []

        for i in range(k):
            while queue:
                if nums[i] >= nums[queue[-1]]:
                    queue.pop()
                else:
                    break
            queue.append(i)

        result.append(nums[queue[0]])

        for i in range(k, len(nums)):
            if queue and queue[0] == (i - k):
                queue.popleft()

            while queue:
                if nums[i] >= nums[queue[-1]]:
                    queue.pop()
                else:
                    break

            queue.append(i)
            result.append(nums[queue[0]])

        return result


def main():
    inputs = [([1, 3, -1, -3, 5, 3, 6, 7], 3), ([1], 1)]

    for nums, k in inputs:
        solution = Solution()
        result = solution.maxSlidingWindow(nums, k)
        print(result)


if __name__ == "__main__":
    main()
