from __future__ import annotations
from typing import List


class Solution:
    def minDeletionSize(self, strs: List[str]) -> int:
        result = 0

        for c in range(len(strs[0])):
            for r in range(1, len(strs)):
                c0 = ord(strs[r - 1][c])
                c1 = ord(strs[r][c])
                if c0 > c1:
                    result += 1
                    break

        return result


def main():
    inputs: list[list[str]] = [["cba", "daf", "ghi"], ["a", "b"], ["zyx", "wvu", "tsr"]]

    solution = Solution()
    for strs in inputs:
        result = solution.minDeletionSize(strs)
        print(result)


if __name__ == "__main__":
    main()
