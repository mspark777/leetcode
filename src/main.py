"""
main
"""

from typing import Optional
from solution import Solution


class Input:
    wrods: list[str]
    pattern: str
    def __init__(self, words: list[str], pattern: str):
        self.words = words
        self.pattern = pattern

def main():
    inputs: list[Input] = [
            Input(
                words = ["abc", "deq", "mee", "aqq", "dkd", "ccc"],
                pattern = "abb"
            ),
            Input(
                words = ["a", "b", "c"],
                pattern = "a"
            )
    ]

    sol = Solution()
    for i in inputs:
        result = sol.findAndReplacePattern(i.words, i.pattern)
        print(result)



if __name__ == "__main__":
    main()
