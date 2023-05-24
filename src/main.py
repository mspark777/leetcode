from typing import List
from queue import PriorityQueue


class Solution:
    def maxScore(self, nums1: List[int], nums2: List[int], k: int) -> int:
        N = len(nums1)
        pairs = [(nums1[i], nums2[i]) for i in range(N)]
        pairs.sort(key=lambda a: -a[1])
        queue = PriorityQueue()
        topk_sum = 0
        for i in range(k):
            topk_sum += pairs[i][0]
            queue.put(pairs[i][0])

        result = topk_sum * pairs[k - 1][1]

        for i in range(k, N):
            topk_sum += pairs[i][0] - queue.get()
            queue.put(pairs[i][0])

            result = max(result, topk_sum * pairs[i][1])

        return result


class Input:
    nums1: list[int]
    nums2: list[int]
    k: int

    def __init__(self, nums1: list[int], nums2: list[int], k: int):
        self.nums1 = nums1
        self.nums2 = nums2
        self.k = k


def main():
    inputs = [
        Input(nums1=[1, 3, 3, 2], nums2=[2, 1, 3, 4], k=3),
        Input(nums1=[4, 2, 3, 1, 1], nums2=[7, 5, 10, 9, 6], k=1),
    ]

    for input in inputs:
        solution = Solution()
        result = solution.maxScore(input.nums1, input.nums2, input.k)
        print(result)


if __name__ == "__main__":
    main()
