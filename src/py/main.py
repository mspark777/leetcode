class Solution:
    def numDifferentIntegers(self, word: str) -> int:
        s = "".join(c if c.isdigit() else " " for c in word)
        return len(set(map(int, s.split())))


class Input:
    word: str

    def __init__(self, word: str):
        self.word = word


def main():
    inputs = [
        Input("a123bc34d8ef34"),
        Input("leet1234code234"),
        Input("a1b01c001"),
    ]

    for input in inputs:
        result = Solution().numDifferentIntegers(input.word)
        print(result)


if __name__ == "__main__":
    main()
