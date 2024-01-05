from __future__ import annotations


class Solution:
    def convertToBase7(self, num: int) -> str:
        if num == 0:
            return "0"

        sign = False
        if num < 0:
            sign = True
            num = -num
        nums: list[str] = []

        while num > 0:
            nums.append(str(num % 7))
            num //= 7

        if sign:
            nums.append("-")

        nums.reverse()
        return "".join(nums)


def main():
    input = (100, -7, 0)

    for num in input:
        result = Solution().convertToBase7(num)
        print(result)


if __name__ == "__main__":
    main()
