from typing import List
from functools import reduce


class Solution:
    def arraySign(self, nums: List[int]) -> int:
        result = reduce(lambda a, b: a * b, nums)
        if result < 0:
            return -1
        elif result > 0:
            return 1

        return 0


def main():
    inputs = [[-1, -2, -3, -4, 3, 2, 1], [1, 5, 0, 2, -3], [-1, 1, -1, 1, -1]]

    for nums in inputs:
        solution = Solution()
        result = solution.arraySign(nums)
        print(result)


if __name__ == "__main__":
    main()
