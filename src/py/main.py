from __future__ import annotations
from typing import List


class Solution:
    def vowelStrings(self, words: List[str], left: int, right: int) -> int:
        vowels = set(["a", "e", "i", "o", "u"])

        result = 0
        for i in range(left, right + 1):
            word = words[i]
            if word[0] in vowels and word[-1] in vowels:
                result += 1

        return result


def main():
    inputs = ((["are", "amy", "u"], 0, 2), (["hey", "aeo", "mu", "ooo", "artro"], 1, 4))

    for words, left, right in inputs:
        result = Solution().vowelStrings(words, left, right)
        print(result)


if __name__ == "__main__":
    main()
