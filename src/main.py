from __future__ import annotations


class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if (str1 + str2) != (str2 + str1):
            return ""

        gcd = self.gcd(len(str1), len(str2))
        return str1[0:gcd]

    def gcd(self, x: int, y: int) -> int:
        return self.gcd(y, x % y) if y != 0 else x


def main():
    inputs: list[list[str]] = [["ABCABC", "ABC"], ["ABABAB", "ABAB"], ["LEET", "CODE"]]

    for [str1, str2] in inputs:
        solution = Solution()
        result = solution.gcdOfStrings(str1, str2)
        print(result)


if __name__ == "__main__":
    main()
