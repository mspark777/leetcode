class Solution:
    def secondHighest(self, s: str) -> int:
        first = -1
        result = -1
        for c in s:
            if not c.isdigit():
                continue
            n = int(c)
            if n > first:
                result = first
                first = n
            elif n > result and n < first:
                result = n

        return result


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs = [
        Input("dfa12321afd"),
        Input("abc1111"),
    ]

    for input in inputs:
        result = Solution().secondHighest(input.s)
        print(result)


if __name__ == "__main__":
    main()
