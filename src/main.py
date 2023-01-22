from __future__ import annotations
from typing import List


class Solution:
    def partition(self, s: str) -> List[List[str]]:
        l = len(s)
        dp = [[False for j in range(l)] for i in range(l)]
        result: list[list[str]] = []
        self.dfs(result, s, 0, [], dp)
        return result

    def dfs(
        self,
        result: list[list[str]],
        s: str,
        start: int,
        current: list[str],
        dp: list[list[bool]],
    ):
        if start >= len(s):
            result.append(current.copy())

        for end in range(start, len(s)):
            check = (s[start] == s[end]) and (
                ((end - start) <= 2) or dp[start + 1][end - 1]
            )
            if check:
                dp[start][end] = True
                current.append(s[start : end + 1])
                self.dfs(result, s, end + 1, current, dp)
                current.pop()


def main():
    inputs: list[str] = ["aab", "a"]
    for s in inputs:
        solution = Solution()
        result = solution.partition(s)
        print(result)


if __name__ == "__main__":
    main()
