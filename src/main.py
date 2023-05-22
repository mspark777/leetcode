from typing import List
from collections import Counter


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        counts = list(Counter(nums).items())
        counts.sort(key=lambda a: -a[1])
        return [c[0] for c in counts[0:k]]


def main():
    inputs = [([1, 1, 1, 2, 2, 3], 2), ([1], 1)]

    for nums, k in inputs:
        solution = Solution()
        result = solution.topKFrequent(nums, k)
        print(result)


if __name__ == "__main__":
    main()
