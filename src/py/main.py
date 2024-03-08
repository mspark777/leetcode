from __future__ import annotations
from typing import List


class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        frequencies = [0] * 101
        max_frequency = 0
        result = 0
        for num in nums:
            frequencies[num] += 1
            frequency = frequencies[num]
            if frequency > max_frequency:
                result = frequency
                max_frequency = frequency
            elif frequency == max_frequency:
                result += frequency

        return result


def main():
    input = [[1, 2, 2, 3, 1, 4], [1, 2, 3, 4, 5]]
    for nums in input:
        result = Solution().maxFrequencyElements(nums)
        print(result)


if __name__ == "__main__":
    main()
