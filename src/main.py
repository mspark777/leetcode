from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        a = 0
        b = 0

        for num in nums:
            tempa = (a & ~b & ~num) + (~a & b & num)
            tempb = (~a & b & ~num) + (~a & ~b & num)

            a = tempa
            b = tempb

        return a | b


def main():
    inputs = [[2, 2, 3, 2], [0, 1, 0, 1, 0, 1, 99]]

    for nums in inputs:
        solution = Solution()
        result = solution.singleNumber(nums)
        print(result)


if __name__ == "__main__":
    main()
