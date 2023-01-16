from __future__ import annotations


class Solution:
    def findTheDifference(self, s: str, t: str) -> str:
        sum = 0

        for c in t:
            sum ^= ord(c)
        for c in s:
            sum ^= ord(c)

        return chr(sum)


def main():
    inputs: list[list[str]] = [["abcd", "abcde"], ["", "y"]]

    for [s, t] in inputs:
        solution = Solution()
        result = solution.findTheDifference(s, t)
        print(result)


if __name__ == "__main__":
    main()
