from typing import List


class Solution:
    def isPrefixString(self, s: str, words: List[str]) -> bool:
        w = ""
        for word in words:
            w += word
            if w == s:
                return True
            elif len(w) > len(s):
                return False

        return False


class Input:
    s: str
    words: List[str]

    def __init__(self, s: str, words: list[str]):
        self.s = s
        self.words = words


def main():
    inputs = [
        Input("iloveleetcode", ["i", "love", "leetcode", "apples"]),
        Input("iloveleetcode", ["apples", "i", "love", "leetcode"]),
    ]

    for input in inputs:
        result = Solution().isPrefixString(input.s, input.words)
        print(result)


if __name__ == "__main__":
    main()
