from __future__ import annotations
from typing import List


class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        result: dict[str, list[int]] = {}
        sequence: list[int] = []
        self.backtrack(nums, 0, sequence, result)
        return list(result.values())

    def backtrack(
        self,
        nums: list[int],
        index: int,
        sequence: list[int],
        result: dict[str, list[int]],
    ):
        if index == len(nums):
            if len(sequence) >= 2:
                key = ",".join(map(str, sequence))
                result[key] = sequence.copy()
        else:
            num = nums[index]
            lastseq = sequence[-1] if len(sequence) > 0 else num
            if lastseq <= num:
                sequence.append(num)
                self.backtrack(nums, index + 1, sequence, result)
                sequence.pop()
            self.backtrack(nums, index + 1, sequence, result)


def main():
    inputs: list[list[int]] = [[4, 6, 7, 7], [4, 4, 3, 2, 1]]
    for nums in inputs:
        solution = Solution()
        result = solution.findSubsequences(nums)
        print(result)


if __name__ == "__main__":
    main()
