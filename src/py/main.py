from __future__ import annotations


class Solution:
    def largestOddNumber(self, num: str) -> str:
        odds = set[str](("1", "3", "5", "7", "9"))
        for i in range(len(num) - 1, -1, -1):
            if num[i] in odds:
                return num[: i + 1]

        return ""


def main():
    inputs = ("52", "4206", "35427")

    for num in inputs:
        result = Solution().largestOddNumber(num)
        print(result)


if __name__ == "__main__":
    main()
