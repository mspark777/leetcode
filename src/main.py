from __future__ import annotations
from typing import Optional, List


class Solution:
    def ladderLength(self, begin_word: str, end_word: str, words: List[str]) -> int:
        word_set = set(words)
        if end_word not in word_set:
            return 0

        visited = set([begin_word])
        queue = [begin_word]

        result = 1
        while queue:
            size = len(queue)
            for _ in range(size):
                word = queue.pop(0)
                if word == end_word:
                    return result

                for j in range(len(word)):
                    for k in range(ord("a"), ord("z") + 1):
                        chs = list(word)
                        chs[j] = chr(k)
                        w = "".join(chs)
                        if w not in word_set:
                            continue
                        elif w in visited:
                            continue

                        queue.append(w)
                        visited.add(w)
            result += 1

        return 0


def main():
    inputs = (
        ("hit", "cog", ["hot", "dot", "dog", "lot", "log", "cog"]),
        ("hit", "cog", ["hot", "dot", "dog", "lot", "log"]),
        (
            "ymain",
            "oecij",
            [
                "ymann",
                "yycrj",
                "oecij",
                "ymcnj",
                "yzcrj",
                "yycij",
                "xecij",
                "yecij",
                "ymanj",
                "yzcnj",
                "ymain",
            ],
        ),
    )

    for begin, end, words in inputs:
        result = Solution().ladderLength(begin, end, words)
        print(result)


if __name__ == "__main__":
    main()
