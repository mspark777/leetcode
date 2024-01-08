from __future__ import annotations
from typing import List


class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        keyboard = dict[str, int]()

        first_row = "qwertyuiop"
        for ch in first_row:
            keyboard[ch] = 1

        second_row = "asdfghjkl"
        for ch in second_row:
            keyboard[ch] = 2

        third_row = "zxcvbnm"
        for ch in third_row:
            keyboard[ch] = 3

        result: list[str] = []
        for word in words:
            row = keyboard[word[0].lower()]
            ok = True
            for ch in word[1:]:
                if keyboard[ch] != row:
                    ok = False
                    break

            if ok:
                result.append(word)

        return result


def main():
    input = (["Hello", "Alaska", "Dad", "Peace"], ["omk"], ["adsdf", "sfd"])

    for words in input:
        result = Solution().findWords(words)
        print(result)


if __name__ == "__main__":
    main()
