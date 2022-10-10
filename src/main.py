from __future__ import annotations
from typing import Optional, List


class Solution:
    def breakPalindrome(self, palindrome: str) -> str:
        slen = len(palindrome)
        if slen <= 1:
            return ""

        for i in range(slen // 2):
            ch = palindrome[i]
            if ch != "a":
                return "{}a{}".format(palindrome[:i], palindrome[i + 1 :])

        return "{}b".format(palindrome[:-1])


def main():
    inputs: list[str] = ["abccba", "a", "aba"]

    solution = Solution()
    for palindrome in inputs:
        result = solution.breakPalindrome(palindrome)
        print(result)


if __name__ == "__main__":
    main()
