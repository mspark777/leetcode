class Solution:
    def minTimeToType(self, word: str) -> int:
        result = len(word)
        prev = "a"

        for ch in word:
            diff = abs(ord(ch) - ord(prev))
            result += min(diff, 26 - diff)
            prev = ch

        return result


class Input:
    word: str

    def __init__(self, word: str):
        self.word = word


def main():
    inputs = [
        Input("abc"),
        Input("bza"),
        Input("zjpc"),
    ]

    for input in inputs:
        result = Solution().minTimeToType(input.word)
        print(result)


if __name__ == "__main__":
    main()
