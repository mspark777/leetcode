from __future__ import annotations
from typing import Optional, List


class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        if s == "":
            return True
        elif t == "":
            return False

        if s[0] == t[0]:
            return self.isSubsequence(s[1:], t[1:])

        return self.isSubsequence(s, t[1:])


def main():
    inputs = [("abc", "ahbgdc"), ("axc", "ahbgdc")]

    for s, t in inputs:
        solution = Solution()
        result = solution.isSubsequence(s, t)
        print(result)


if __name__ == "__main__":
    main()
