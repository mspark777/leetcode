from __future__ import annotations
from typing import Optional, List


class Solution:
    def intToRoman(self, num: int) -> str:
        M = ["", "M", "MM", "MMM"]
        C = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]
        X = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]
        I = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]

        mi = num // 1000
        ci = (num % 1000) // 100
        xi = (num % 100) // 10
        ii = num % 10

        return f"{M[mi]}{C[ci]}{X[xi]}{I[ii]}"


def main():
    inputs: list[int] = [3, 58, 1994]

    solution = Solution()
    for num in inputs:
        result = solution.intToRoman(num)
        print(result)


if __name__ == "__main__":
    main()
