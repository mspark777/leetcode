from __future__ import annotations
from typing import Optional, List


class Solution:
    def checkIfPangram(self, sentence: str) -> bool:
        ACODE = ord("a")
        bits = 0

        for ch in sentence:
            code = ord(ch)
            offset = code - ACODE
            bit = 1 << offset

            bits |= bit

        return bits == 0x03FFFFFF


def main():
    inputs: list[str] = ["thequickbrownfoxjumpsoverthelazydog", "leetcode"]

    solution = Solution()
    for sentence in inputs:
        result = solution.checkIfPangram(sentence)
        print(result)


if __name__ == "__main__":
    main()
