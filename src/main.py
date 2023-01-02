from __future__ import annotations
from typing import List


class Solution:
    def detectCapitalUse(self, word: str) -> bool:
        ACODE = ord("A")
        ZCODE = ord("Z")
        count = 0
        begin = -1

        for i, ch in enumerate(word):
            code = ord(ch)
            if (ACODE <= code) and (code <= ZCODE):
                count += 1
                begin = i

        return (count < 1) or (count == len(word)) or ((count == 1) and (begin == 0))


def main():
    inputs: list[str] = ["USA", "Google", "leetcode", "FlaG"]

    solution = Solution()
    for word in inputs:
        result = solution.detectCapitalUse(word)
        print(result)


if __name__ == "__main__":
    main()
