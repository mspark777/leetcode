class Solution:
    def reformat(self, s: str) -> str:
        letters = [c for c in s if c.isalpha()]
        digits = [c for c in s if c.isdigit()]
        if abs(len(letters) - len(digits)) > 1:
            return ""
        elif len(letters) > len(digits):
            s = "".join([f"{l}{d}" for l, d in zip(letters, digits)])
            return s + letters[-1]
        elif len(letters) < len(digits):
            s = "".join([f"{d}{l}" for l, d in zip(letters, digits)])
            return s + digits[-1]

        return "".join([f"{l}{d}" for l, d in zip(letters, digits)])


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs = [Input("a0b1c2"), Input("leetcode"), Input("1229857369")]

    for input in inputs:
        result = Solution().reformat(input.s)
        print(result)


if __name__ == "__main__":
    main()
