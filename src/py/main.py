from typing import  List


class Solution:
    def zeroFilledSubarray(self, nums: List[int]) -> int:
        result = 0
        sub = 0

        for num in nums:
            if num == 0:
                sub += 1
            else:
                sub = 0

            result += sub

        return result


def main():
    inputs: list[list[int]] = [
        [1, 3, 0, 0, 2, 0, 0, 4],
        [0, 0, 0, 2, 0, 0],
        [2, 10, 2019]
    ]

    for nums in inputs:
        solution = Solution()
        result = solution.zeroFilledSubarray(nums)
        print(result)


if __name__ == "__main__":
    main()
