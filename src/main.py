from __future__ import annotations
from typing import Optional, List


class Solution:
    def maxLength(self, arr: List[str]) -> int:
        dp: list[set[str]] = [set()]
        for str in arr:
            memo = set(str)
            if len(memo) < len(str):
                continue

            for d in dp[:]:
                if memo & d:
                    continue
                else:
                    dp.append(memo | d)

        return max(len(a) for a in dp)


def main():
    inputs: list[list[str]] = [
        ["un", "iq", "ue"],
        ["cha", "r", "act", "ers"],
        ["abcdefghijklmnopqrstuvwxyz"],
    ]

    solution = Solution()
    for arr in inputs:
        result = solution.maxLength(arr)
        print(result)


if __name__ == "__main__":
    main()
