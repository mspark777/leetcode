from __future__ import annotations


class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        if s == "":
            return True

        si = 0

        for c in t:
            if s[si] == c:
                si += 1

            if len(s) == si:
                return True

        return False


def main():
    inputs: list[list[str]] = [["abc", "ahbgdc"], ["axc", "ahbgdc"]]
    for [s, t] in inputs:
        solution = Solution()
        result = solution.isSubsequence(s, t)
        print(result)


if __name__ == "__main__":
    main()
