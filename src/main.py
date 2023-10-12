from __future__ import annotations
from typing import Optional, List


# """
# This is MountainArray's API interface.
# You should not implement it, or speculate about its implementation
# """
# class MountainArray:
#    def get(self, index: int) -> int:
#    def length(self) -> int:


class Solution:
    def findInMountainArray(self, target: int, mountain_arr: "MountainArray") -> int:
        # I don't like this problem.
        length = mountain_arr.length()
        low = 1
        high = length - 2
        while low != high:
            test_index = (low + high) >> 1
            curr = mountain_arr.get(test_index)
            next = mountain_arr.get(test_index + 1)

            if curr < next:
                if curr == target:
                    return test_index
                if next == target:
                    return test_index + 1
                low = test_index + 1
            else:
                high = test_index

        peak_index = low

        low = 0
        high = peak_index
        while low <= high:
            test_index = (low + high) >> 1
            curr = mountain_arr.get(test_index)

            if curr == target:
                return test_index
            elif curr < target:
                low = test_index + 1
            else:
                high = test_index - 1

        low = peak_index + 1
        high = length - 1
        while low <= high:
            test_index = (low + high) >> 1
            curr = mountain_arr.get(test_index)

            if curr == target:
                return test_index
            elif curr > target:
                low = test_index + 1
            else:
                high = test_index - 1

        return -1


def main():
    inputs = (([1, 2, 3, 4, 5, 3, 1], 3), ([0, 1, 2, 4, 2, 1], 3))

    for target, mountain_arr in inputs:
        result = Solution().findInMountainArray(target, mountain_arr)
        print(result)


if __name__ == "__main__":
    main()
