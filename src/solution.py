"""
solution
"""
from __future__ import annotations
from typing import  Optional

class Solution:
    def findAndReplacePattern(self, words: list[str], pattern: str) -> list[str]:
        result: list[str] = []
        for word in words:
            if self.find_pattern(word, pattern):
                result.append(word)
        return result

    def find_pattern(self, word: str, pattern: str) -> bool:
        if len(word) != len(pattern):
            return False

        wmap: dict[str, str] = {}
        pmap: dict[str, str] = {}

        for i, wc in enumerate(word):
            pc = pattern[i]

            if wc not in wmap:
                wmap[wc] = pc

            if pc not in pmap:
                pmap[pc] = wc

            if wmap[wc] != pc:
                return False

            if pmap[pc] != wc:
                return False

        return True
