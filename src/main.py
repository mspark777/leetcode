from __future__ import annotations
from typing import Optional, List


class Solution:
    def numDecodings(self, s: str) -> int:
        slen = len(s)
        if slen == 0:
            return 0

        if s[0] == "0":
            return 0

        if slen == 1:
            return 1

        ZERO = ord("0")

        d1 = 1
        d2 = 1

        for i in range(1, slen):
            code1 = ord(s[i]) - ZERO
            code0 = ((ord(s[i - 1]) - ZERO) * 10) + code1

            n = 0
            if code1 != 0:
                n += d1

            if (10 <= code0) and (code0 <= 26):
                n += d2

            d2 = d1
            d1 = n

        return d1


def main():
    inputs: list[str] = ["12", "226", "06"]

    solution = Solution()
    for input in inputs:
        result = solution.numDecodings(input)
        print(result)


if __name__ == "__main__":
    main()
