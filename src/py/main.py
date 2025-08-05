class Solution:
    def freqAlphabets(self, s: str) -> str:
        result: list[str] = []
        i = 0
        while i < len(s):
            if ((i + 2) < len(s)) and (s[i + 2] == "#"):
                val = int(s[i : i + 2])
                result.append(chr(val + 96))
                i += 3
            else:
                result.append(chr(int(s[i]) + 96))
                i += 1

        return "".join(result)


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs = [Input("10#11#12"), Input("1326#")]

    for input in inputs:
        result = Solution().freqAlphabets(input.s)
        print(result)


if __name__ == "__main__":
    main()
