"""
main
"""

from __future__ import annotations
from typing import Optional

class Solution:
    def findSubstring(self, s: str, words: list[str]) -> list[int]:
        words_len = len(words)
        word_len = len(words[0])

        result: list[int] = []
        word_count: dict[str, int] = dict()
        for word in words:
            if word not in word_count:
                word_count[word] = 1
            else:
                word_count[word] += 1

        last_window_index = len(s) - (words_len * word_len)
        for i in range(last_window_index + 1):
            tword_count = dict(word_count)

            for j in range(i, len(s), word_len):
                str = s[j:j+word_len]
                if str not in tword_count:
                    break

                cnt = tword_count[str]
                if cnt == 0:
                    break
                else:
                    if cnt == 1:
                        tword_count.pop(str)
                    else:
                        tword_count[str] = cnt - 1
            if len(tword_count) == 0:
                result.append(i)

        return result


class Input:
    s: str
    words: list[str]

    def __init__(self, s: str, words: list[str]):
        self.s = s
        self.words = words

def main():
    inputs: list[Input] = [
            Input("barfoothefoobarman", ["foo", "bar"]),
            Input("wordgoodgoodgoodbestword", ["word", "good", "best", "word"]),
            Input("barfoofoobarthefoobarman", ["bar", "foo", "the"]),
    ]

    s = Solution()
    for i in inputs:
        result = s.findSubstring(i.s, i.words)
        print(result)

if __name__ == "__main__":
    main()
