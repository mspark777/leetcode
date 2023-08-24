from __future__ import annotations
from typing import List


class Solution:
    def fullJustify(self, words: List[str], max_width: int) -> List[str]:
        result: list[str] = []
        i = 0
        while i < len(words):
            line = self.get_words(i, words, max_width)
            i += len(line)
            result.append(self.create_line(line, i, words, max_width))

        return result

    def get_words(self, i: int, words: list[str], max_width: int) -> list[str]:
        line: list[str] = []
        line_len = 0

        while i < len(words):
            word = words[i]
            width = line_len + len(word)
            if width > max_width:
                break

            line.append(word)
            line_len += len(word) + 1
            i += 1

        return line

    def create_line(
        self, line: list[str], i: int, words: list[str], max_width: int
    ) -> str:
        base_length = -1
        for word in line:
            base_length += len(word) + 1

        extra_spaces = max_width - base_length
        if (len(line) == 1) or (len(words) == i):
            return " ".join(line) + (" " * extra_spaces)

        word_count = len(line) - 1
        spaces_per_word = extra_spaces // word_count
        needs_extra_space = extra_spaces % word_count
        for j in range(needs_extra_space):
            line[j] += " "

        for j in range(word_count):
            line[j] += " " * spaces_per_word

        return " ".join(line)


def main():
    inputs = [
        (["This", "is", "an", "example", "of", "text", "justification."], 16),
        (["What", "must", "be", "acknowledgment", "shall", "be"], 16),
        (
            [
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do",
            ],
            20,
        ),
    ]

    for words, max_width in inputs:
        solution = Solution()
        result = solution.fullJustify(words, max_width)
        print(result)


if __name__ == "__main__":
    main()
