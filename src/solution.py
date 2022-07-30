"""
solution
"""
from __future__ import annotations
from typing import  Optional

LETTER_COUNT = 26
ACODE = ord('a')

class Solution:
    def wordSubsets(self, words1: list[str], words2: list[str]) -> list[str]:
        counts2 = self.get_counts("")
        for word in words2:
            counts3 = self.get_counts(word)
            for i in range(LETTER_COUNT):
                counts2[i] = max(counts2[i], counts3[i])

        result: list[str] = []
        for word in words1:
            counts1 = self.get_counts(word)
            ok = True
            for i in range(LETTER_COUNT):
                if counts1[i] < counts2[i]:
                    ok = False
                    break

            if ok:
                result.append(word)

        return result

    def get_counts(self, word: str) -> list[int]:
        counts = [0 for i in range(LETTER_COUNT)]
        for ch in word:
            i = ord(ch) - ACODE
            counts[i] += 1

        return counts
